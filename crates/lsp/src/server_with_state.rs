use std::{ops::ControlFlow, sync::Arc};

use futures::future::BoxFuture;

use async_lsp::{
    ClientSocket, LanguageServer, ResponseError, Result,
    lsp_types::{
        CodeAction, CodeActionOrCommand, CodeActionParams, CompletionItem, CompletionParams,
        CompletionResponse, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
        DidSaveTextDocumentParams, GotoDefinitionParams, GotoDefinitionResponse, Hover,
        HoverParams, InitializeParams, InitializeResult, Location, PositionEncodingKind,
        PrepareRenameResponse, ReferenceParams, RenameParams, TextDocumentPositionParams,
        TextDocumentSyncCapability, TextDocumentSyncKind, WorkspaceEdit,
    },
};

use crate::{server_state::ServerState, server_trait::Server};

/**
    The low-level language server implementation that automatically
    manages documents and forwards requests to the underlying server.

    Supports incremental updates of documents where possible, falling
    back to other implementations whenever incremental updates fail.
*/
pub(crate) struct LanguageServerWithState<T: Server> {
    server: Arc<T>,
    state: ServerState,
}

impl<T: Server> LanguageServerWithState<T> {
    pub(crate) fn new(client: ClientSocket, server: T) -> Self {
        let server = Arc::new(server);
        let state = ServerState::new(client);
        Self { server, state }
    }
}

impl<T: Server + Send + Sync + 'static> LanguageServer for LanguageServerWithState<T> {
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

    // Document callbacks & updating

    fn did_open(&mut self, params: DidOpenTextDocumentParams) -> ControlFlow<Result<()>> {
        self.state.handle_document_open(params)
    }

    fn did_change(&mut self, params: DidChangeTextDocumentParams) -> ControlFlow<Result<()>> {
        self.state.handle_document_change(params)
    }

    fn did_save(&mut self, params: DidSaveTextDocumentParams) -> ControlFlow<Result<()>> {
        self.state.handle_document_save(params)
    }

    // Forwarding for: Hover, Completion, Code Action

    fn hover(
        &mut self,
        params: HoverParams,
    ) -> BoxFuture<'static, Result<Option<Hover>, Self::Error>> {
        let server = Arc::clone(&self.server);
        let state = self.state.clone();
        Box::pin(async move { Ok(server.hover(state, params).await?) })
    }

    fn completion(
        &mut self,
        params: CompletionParams,
    ) -> BoxFuture<'static, Result<Option<CompletionResponse>, Self::Error>> {
        let server = Arc::clone(&self.server);
        let state = self.state.clone();
        Box::pin(async move { Ok(server.completion(state, params).await?) })
    }

    fn completion_item_resolve(
        &mut self,
        item: CompletionItem,
    ) -> BoxFuture<'static, Result<CompletionItem, Self::Error>> {
        let server = Arc::clone(&self.server);
        let state = self.state.clone();
        Box::pin(async move { Ok(server.completion_resolve(state, item).await?) })
    }

    fn code_action(
        &mut self,
        params: CodeActionParams,
    ) -> BoxFuture<'static, Result<Option<Vec<CodeActionOrCommand>>, Self::Error>> {
        let server = Arc::clone(&self.server);
        let state = self.state.clone();
        Box::pin(async move { Ok(server.code_action(state, params).await?) })
    }

    fn code_action_resolve(
        &mut self,
        item: CodeAction,
    ) -> BoxFuture<'static, Result<CodeAction, Self::Error>> {
        let server = Arc::clone(&self.server);
        let state = self.state.clone();
        Box::pin(async move { Ok(server.code_action_resolve(state, item).await?) })
    }

    // Forwarding for: Definition, References, Rename

    fn definition(
        &mut self,
        params: GotoDefinitionParams,
    ) -> BoxFuture<'static, Result<Option<GotoDefinitionResponse>, Self::Error>> {
        let server = Arc::clone(&self.server);
        let state = self.state.clone();
        Box::pin(async move { Ok(server.definition(state, params).await?) })
    }

    fn references(
        &mut self,
        params: ReferenceParams,
    ) -> BoxFuture<'static, Result<Option<Vec<Location>>, Self::Error>> {
        let server = Arc::clone(&self.server);
        let state = self.state.clone();
        Box::pin(async move { Ok(server.references(state, params).await?) })
    }

    fn rename(
        &mut self,
        params: RenameParams,
    ) -> BoxFuture<'static, Result<Option<WorkspaceEdit>, Self::Error>> {
        let server = Arc::clone(&self.server);
        let state = self.state.clone();
        Box::pin(async move { Ok(server.rename(state, params).await?) })
    }

    fn prepare_rename(
        &mut self,
        params: TextDocumentPositionParams,
    ) -> BoxFuture<'static, Result<Option<PrepareRenameResponse>, Self::Error>> {
        let server = Arc::clone(&self.server);
        let state = self.state.clone();
        Box::pin(async move { Ok(server.rename_prepare(state, params).await?) })
    }
}
