use std::collections::HashMap;
use std::io;
use std::ops::ControlFlow;

use futures::future::BoxFuture;
use ropey::Rope;

use async_lsp::{
    Error, LanguageServer, ResponseError, Result,
    lsp_types::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
        InitializeParams, InitializeResult, PositionEncodingKind, TextDocumentSyncCapability,
        TextDocumentSyncKind, Url,
    },
};

use crate::{document::Document, server_trait::Server};

/**
    The low-level language server implementation that automatically
    manages documents and forwards requests to the underlying server.

    Supports incremental updates of documents where possible, falling
    back to other implementations whenever incremental updates fail.
*/
pub(crate) struct LanguageServerWithDocuments<T: Server> {
    server: T,
    documents: HashMap<Url, Document>,
}

impl<T: Server> LanguageServerWithDocuments<T> {
    pub(crate) fn new(server: T) -> Self {
        let documents = HashMap::new();
        Self { server, documents }
    }
}

impl<T: Server> LanguageServer for LanguageServerWithDocuments<T> {
    type Error = ResponseError;
    type NotifyResult = ControlFlow<async_lsp::Result<()>>;

    fn initialize(
        &mut self,
        params: InitializeParams,
    ) -> BoxFuture<'static, Result<InitializeResult, Self::Error>> {
        let mut result = InitializeResult {
            server_info: T::server_info(),
            capabilities: T::server_capabilities(params.capabilities).unwrap_or_default(),
        };

        result.capabilities.position_encoding = Some(PositionEncodingKind::UTF32);
        result.capabilities.text_document_sync = Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        ));

        Box::pin(async move { Ok(result) })
    }

    fn did_open(&mut self, params: DidOpenTextDocumentParams) -> ControlFlow<Result<()>> {
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

    fn did_change(&mut self, params: DidChangeTextDocumentParams) -> ControlFlow<Result<()>> {
        let Some(doc) = self.documents.get_mut(&params.text_document.uri) else {
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

    fn did_save(&mut self, params: DidSaveTextDocumentParams) -> ControlFlow<Result<()>> {
        let Some(doc) = self.documents.get_mut(&params.text_document.uri) else {
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
