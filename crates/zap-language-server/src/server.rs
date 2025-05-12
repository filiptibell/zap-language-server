use async_language_server::{
    lsp_types::{
        ClientCapabilities, CompletionOptions, CompletionParams, CompletionResponse, Hover,
        HoverParams, HoverProviderCapability, ServerCapabilities, ServerInfo, Url,
    },
    server::{Server, ServerResult, ServerState},
    tree_sitter::Language,
};

use crate::{
    completions::{completion_for_options, completion_trigger_characters},
    hovers::hover_for_options,
};

#[derive(Debug, Clone)]
pub struct ZapLanguageServer {}

impl ZapLanguageServer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ZapLanguageServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Server for ZapLanguageServer {
    fn server_info() -> Option<ServerInfo> {
        Some(ServerInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        })
    }

    fn server_capabilities(_: ClientCapabilities) -> Option<ServerCapabilities> {
        Some(ServerCapabilities {
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(true),
                trigger_characters: Some(completion_trigger_characters()),
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    fn determine_tree_sitter_language(_: &Url, language: &str) -> Option<Language> {
        if language.trim().eq_ignore_ascii_case("zap") {
            Some(tree_sitter_zap::LANGUAGE.into())
        } else {
            None
        }
    }

    async fn hover(&self, state: ServerState, params: HoverParams) -> ServerResult<Option<Hover>> {
        let url = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let Some(doc) = state.document(&url) else {
            return Ok(None);
        };
        let Some(node) = doc.node_at_position(pos) else {
            return Ok(None);
        };
        let Some(parent) = node.parent() else {
            return Ok(None);
        };

        Ok(hover_for_options(&doc, &pos, &node, &parent))
    }

    async fn completion(
        &self,
        state: ServerState,
        params: CompletionParams,
    ) -> ServerResult<Option<CompletionResponse>> {
        let url = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        let Some(doc) = state.document(&url) else {
            return Ok(None);
        };
        let Some(node) = doc.node_at_position(pos) else {
            return Ok(None);
        };
        let Some(parent) = node.parent() else {
            return Ok(None);
        };

        Ok(completion_for_options(&doc, &pos, &node, &parent))
    }
}
