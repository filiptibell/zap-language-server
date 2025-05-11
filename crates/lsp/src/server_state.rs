#![allow(clippy::needless_pass_by_value)]

use std::{ops::ControlFlow, sync::Arc};

use dashmap::DashMap;
use ropey::Rope;

use async_lsp::{
    ClientSocket, Result,
    lsp_types::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, Url,
    },
};

#[cfg(feature = "tree-sitter")]
use tree_sitter::Parser;

use crate::{document::Document, server::Server};

/**
    Managed state for an LSP server.

    Provides access to and automatically tracks the connected
    client, as well as opened documents and their changes.
*/
#[derive(Debug, Clone)]
pub struct ServerState {
    client: ClientSocket,
    documents: Arc<DashMap<Url, Document>>,
}

impl ServerState {
    /**
        Gets a handle to the client connected to the server.

        Can be used to send requests and notifications to the client.
    */
    #[must_use]
    pub fn client(&self) -> ClientSocket {
        self.client.clone()
    }

    /**
        Gets a snapshot of a document by its URL.

        This will return the document exactly as it was
        at the time of calling this method - any further
        modifications such as saves or edits will not be
        reflected in the returned document or its contents.

        Returns `None` if the document is not found.
    */
    #[must_use]
    pub fn document(&self, url: &Url) -> Option<Document> {
        let doc = self.documents.get(url)?;
        Some(doc.clone())
    }
}

// Private implementation

impl ServerState {
    pub(crate) fn new(client: ClientSocket) -> Self {
        let documents = Arc::new(DashMap::new());
        Self { client, documents }
    }

    fn insert_document<T: Server>(&self, url: Url, text: String, version: i32, language: String) {
        #[cfg(feature = "tree-sitter")]
        let mut tree_sitter_lang = T::determine_tree_sitter_language(&url, language.as_str());

        #[cfg(feature = "tree-sitter")]
        let tree_sitter_tree = if let Some(lang) = tree_sitter_lang.as_ref() {
            let mut parser = Parser::new();
            if parser.set_language(lang).is_ok() {
                parser.parse(&text, None)
            } else {
                tree_sitter_lang.take();
                None
            }
        } else {
            None
        };

        self.documents.insert(
            url.clone(),
            Document {
                uri: url,
                text: Rope::from(text),
                version,
                language,
                #[cfg(feature = "tree-sitter")]
                tree_sitter_lang,
                #[cfg(feature = "tree-sitter")]
                tree_sitter_tree,
            },
        );
    }

    pub(crate) fn handle_document_open<T: Server>(
        &mut self,
        params: DidOpenTextDocumentParams,
    ) -> ControlFlow<Result<()>> {
        self.insert_document::<T>(
            params.text_document.uri,
            params.text_document.text,
            params.text_document.version,
            params.text_document.language_id,
        );

        ControlFlow::Continue(())
    }

    pub(crate) fn handle_document_change<T: Server>(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> ControlFlow<Result<()>> {
        let Some(mut doc) = self.documents.get_mut(&params.text_document.uri) else {
            return ControlFlow::Continue(());
        };

        doc.version = params.text_document.version;

        // Try to perform an incremental update on the document contents, using the changes
        let mut incremental_update_failed = false;
        for change in params.content_changes {
            let Some(range) = change.range else { continue };

            // Try to find the character indices where the change starts and ends
            let start_idx = doc
                .text
                .try_line_to_char(range.start.line as usize)
                .map(|idx| idx + range.start.character as usize);
            let end_idx = doc
                .text
                .try_line_to_char(range.end.line as usize)
                .map(|idx| idx + range.end.character as usize)
                .and_then(|c| {
                    if c > doc.text.len_chars() {
                        Err(ropey::Error::CharIndexOutOfBounds(c, doc.text.len_chars()))
                    } else {
                        Ok(c)
                    }
                });

            // Try to incrementally update, or exit early if it fails
            match (start_idx, end_idx) {
                (Ok(start_idx), Err(_)) => {
                    if doc.text.try_remove(start_idx..).is_err()
                        || doc.text.try_insert(start_idx, &change.text).is_err()
                    {
                        incremental_update_failed = true;
                        break;
                    }
                }
                (Ok(start_idx), Ok(end_idx)) => {
                    if doc.text.try_remove(start_idx..end_idx).is_err()
                        || doc.text.try_insert(start_idx, &change.text).is_err()
                    {
                        incremental_update_failed = true;
                        break;
                    }
                }
                (Err(_), _) => {
                    incremental_update_failed = true;
                    break;
                }
            }

            // Perform incremental edit on the syntax tree as well, if enabled
            #[cfg(feature = "tree-sitter")]
            if let Some(tree) = doc.tree_sitter_tree.as_mut() {}
        }

        // If the incremental update was successful, and we applied edits to the syntax
        // tree, we must finalize those changes by parsing using tree-sitter once again
        #[cfg(feature = "tree-sitter")]
        if !incremental_update_failed {
            if let Some(tree) = doc.tree_sitter_tree.as_ref() {
                let mut parser = doc.parser().expect("has tree - must have parser");
                let updated_tree = parser.parse(doc.text_contents(), Some(tree));
                doc.tree_sitter_tree = updated_tree;
            }
        }

        // If the incremental update failed, we will re-insert the entire file instead
        // Note that we must first drop the document reference to prevent a deadlock
        if incremental_update_failed {
            let uri = doc.uri.clone();
            let version = doc.version();
            let language = doc.language.clone();

            drop(doc);

            // NOTE: We must read the contents of the file synchronously
            // as the fallback here, since notification handlers are actually
            // synchronous both according to LSP spec and the async-lsp crate
            if let Ok(text) = std::fs::read_to_string(uri.path()) {
                self.insert_document::<T>(uri, text, version, language);
            } else {
                self.documents.remove(&uri);
            }
        }

        ControlFlow::Continue(())
    }

    pub(crate) fn handle_document_save<T: Server>(
        &self,
        params: DidSaveTextDocumentParams,
    ) -> ControlFlow<Result<()>> {
        let Some(mut doc) = self.documents.get_mut(&params.text_document.uri) else {
            return ControlFlow::Continue(());
        };

        // NOTE: We must read the contents of the file synchronously
        // as the fallback here, since notification handlers are actually
        // synchronous both according to LSP spec and the async-lsp crate
        doc.text = if let Some(text) = &params.text {
            Rope::from_str(text)
        } else if let Ok(text) = std::fs::read_to_string(params.text_document.uri.path()) {
            Rope::from_str(&text)
        } else {
            self.documents.remove(&params.text_document.uri);
            return ControlFlow::Continue(());
        };

        // Since we just read the entire file contents, we will also
        // re-create the entire tree-sitter tree using those new contents
        #[cfg(feature = "tree-sitter")]
        {
            let mut tree_sitter_lang = T::determine_tree_sitter_language(doc.url(), doc.language());

            let tree_sitter_tree = if let Some(lang) = tree_sitter_lang.as_ref() {
                let mut parser = Parser::new();
                if parser.set_language(lang).is_ok() {
                    parser.parse(doc.text_contents(), None)
                } else {
                    tree_sitter_lang.take();
                    None
                }
            } else {
                None
            };

            doc.tree_sitter_lang = tree_sitter_lang;
            doc.tree_sitter_tree = tree_sitter_tree;
        }

        ControlFlow::Continue(())
    }
}
