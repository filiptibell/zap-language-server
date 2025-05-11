#![allow(clippy::needless_pass_by_value)]

use std::{io, ops::ControlFlow, sync::Arc};

use dashmap::DashMap;
use ropey::Rope;

use async_lsp::{
    ClientSocket, Error, Result,
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

        for change in params.content_changes {
            let Some(range) = change.range else { continue };

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

            match (start_idx, end_idx) {
                (Ok(start_idx), Err(_)) => {
                    result_to_control_flow(doc.text.try_remove(start_idx..))?;
                    result_to_control_flow(doc.text.try_insert(start_idx, &change.text))?;
                }
                (Ok(start_idx), Ok(end_idx)) => {
                    result_to_control_flow(doc.text.try_remove(start_idx..end_idx))?;
                    result_to_control_flow(doc.text.try_insert(start_idx, &change.text))?;
                }
                (Err(_), _) => {
                    self.insert_document::<T>(
                        doc.uri.clone(),
                        change.text,
                        doc.version,
                        doc.language.clone(),
                    );
                }
            }
        }

        ControlFlow::Continue(())
    }

    pub(crate) fn handle_document_save<T: Server>(
        &self,
        params: DidSaveTextDocumentParams,
    ) -> ControlFlow<Result<()>> {
        let Some(mut doc) = self.documents.get_mut(&params.text_document.uri) else {
            return error_to_control_break(format!(
                "Document {} not found",
                params.text_document.uri
            ));
        };

        // NOTE: We must unfortunately read the contents of the file
        // synchronously in the fallback here, since async-lsp will
        // not allow us to use async code in notification handlers
        doc.text = if let Some(text) = &params.text {
            Rope::from_str(text)
        } else {
            let file = match std::fs::File::open(params.text_document.uri.path()) {
                Ok(file) => file,
                Err(err) => return error_to_control_break(err),
            };
            match Rope::from_reader(file) {
                Ok(rope) => rope,
                Err(err) => return error_to_control_break(err),
            }
        };

        // Since we just read the entire file contents, we will also
        // re-compute the entire tree-sitter tree from those new contents
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

fn result_to_control_flow<E>(result: Result<(), E>) -> ControlFlow<Result<()>>
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    match result {
        Ok(()) => ControlFlow::Continue(()),
        Err(err) => error_to_control_break(err),
    }
}

fn error_to_control_break<E>(err: E) -> ControlFlow<Result<()>>
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    ControlFlow::Break(Err(Error::Io(io::Error::new(io::ErrorKind::Other, err))))
}
