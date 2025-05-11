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

use crate::document::Document;

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

    pub(crate) fn handle_document_open(
        &mut self,
        params: DidOpenTextDocumentParams,
    ) -> ControlFlow<Result<()>> {
        self.documents.insert(
            params.text_document.uri.clone(),
            Document {
                uri: params.text_document.uri.clone(),
                text: Rope::from(params.text_document.text),
                lang: params.text_document.language_id,
            },
        );

        ControlFlow::Continue(())
    }

    pub(crate) fn handle_document_change(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> ControlFlow<Result<()>> {
        let Some(mut doc) = self.documents.get_mut(&params.text_document.uri) else {
            return ControlFlow::Continue(());
        };

        for change in params.clone().content_changes {
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
                    *doc = Document {
                        uri: doc.uri.clone(),
                        text: Rope::from(change.text),
                        lang: doc.lang.clone(),
                    }
                }
            }
        }

        ControlFlow::Continue(())
    }

    pub(crate) fn handle_document_save(
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
