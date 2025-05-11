#![allow(unused_variables)]
#![allow(clippy::unused_async)]
#![allow(clippy::must_use_candidate)]

use async_lsp::{
    ErrorCode,
    lsp_types::{
        ClientCapabilities, CodeAction, CodeActionParams, CodeActionResponse, CompletionItem,
        CompletionParams, CompletionResponse, GotoDefinitionParams, GotoDefinitionResponse, Hover,
        HoverParams, Location, PrepareRenameResponse, ReferenceParams, RenameParams,
        ServerCapabilities, ServerInfo, TextDocumentPositionParams, WorkspaceEdit,
    },
};

use crate::{
    result::{ServerError, ServerResult},
    server_state::ServerState,
};

/**
    The main entrypoint to LSP functionality for a server.

    All of the LSP methods in this trait are optional - if implemented,
    the respective capabilities must also be registered using the
    `server_capabilities` function.

    The only exception to this rule are the `*_resolve` methods, which
    default to doing nothing, and simply resolving the item as-is.
*/
pub trait Server {
    fn server_info() -> Option<ServerInfo> {
        None
    }

    fn server_capabilities(client_capabilities: ClientCapabilities) -> Option<ServerCapabilities> {
        None
    }

    // Hover, Completion, Code Action

    fn hover(
        &self,
        state: ServerState,
        params: HoverParams,
    ) -> impl Future<Output = ServerResult<Option<Hover>>> + Send + 'static {
        method_not_implemented("hover")
    }

    fn completion(
        &self,
        state: ServerState,
        params: CompletionParams,
    ) -> impl Future<Output = ServerResult<Option<CompletionResponse>>> + Send + 'static {
        method_not_implemented("completion")
    }

    fn completion_resolve(
        &self,
        state: ServerState,
        item: CompletionItem,
    ) -> impl Future<Output = ServerResult<CompletionItem>> + Send + 'static {
        async move { Ok(item) }
    }

    fn code_action(
        &self,
        state: ServerState,
        params: CodeActionParams,
    ) -> impl Future<Output = ServerResult<Option<CodeActionResponse>>> + Send + 'static {
        method_not_implemented("code_action")
    }

    fn code_action_resolve(
        &self,
        state: ServerState,
        action: CodeAction,
    ) -> impl Future<Output = ServerResult<CodeAction>> + Send + 'static {
        async move { Ok(action) }
    }

    // Definition, References, Rename

    fn definition(
        &self,
        state: ServerState,
        params: GotoDefinitionParams,
    ) -> impl Future<Output = ServerResult<Option<GotoDefinitionResponse>>> + Send + 'static {
        method_not_implemented("definition")
    }

    fn references(
        &self,
        state: ServerState,
        params: ReferenceParams,
    ) -> impl Future<Output = ServerResult<Option<Vec<Location>>>> + Send + 'static {
        method_not_implemented("references")
    }

    fn rename(
        &self,
        state: ServerState,
        params: RenameParams,
    ) -> impl Future<Output = ServerResult<Option<WorkspaceEdit>>> + Send + 'static {
        method_not_implemented("rename")
    }

    fn rename_prepare(
        &self,
        state: ServerState,
        params: TextDocumentPositionParams,
    ) -> impl Future<Output = ServerResult<Option<PrepareRenameResponse>>> + Send + 'static {
        method_not_implemented("rename_prepare")
    }
}

async fn method_not_implemented<T>(name: &'static str) -> Result<T, ServerError> {
    Err(ServerError::rpc(
        ErrorCode::METHOD_NOT_FOUND,
        format!("LSP method '{name}' has not been implemented"),
    ))
}
