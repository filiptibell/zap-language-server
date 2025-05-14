use async_language_server::{
    lsp_types::{
        ClientCapabilities, CompletionItem, CompletionOptions, CompletionParams,
        CompletionResponse, GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverParams,
        HoverProviderCapability, OneOf, ServerCapabilities, ServerInfo, Url,
    },
    server::{Server, ServerResult, ServerState},
    tree_sitter::Language,
};

use crate::{
    completions::{
        completion_for_instances, completion_for_keywords, completion_for_options,
        completion_for_types, completion_trigger_characters,
    },
    definitions::definition_for_types,
    hovers::{hover_for_keywords, hover_for_options, hover_for_properties, hover_for_types},
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
            definition_provider: Some(OneOf::Left(true)),
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
        let Some(node) = doc.node_at_position_named(pos) else {
            tracing::debug!("Missing node for hover at {}:{}", pos.line, pos.character);
            return Ok(None);
        };

        let text = doc.text().byte_slice(node.byte_range());
        tracing::debug!(
            "Getting hover for node at {}:{} with contents '{text}'",
            pos.line,
            pos.character
        );

        Ok(hover_for_keywords(&doc, pos, node)
            .or_else(|| hover_for_types(&doc, pos, node))
            .or_else(|| hover_for_properties(&doc, pos, node))
            .or_else(|| hover_for_options(&doc, pos, node)))
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
        let Some(node) = doc.node_at_position_named(pos) else {
            tracing::debug!(
                "Missing node for completion at {}:{}",
                pos.line,
                pos.character
            );
            return Ok(None);
        };

        let text = doc.text().byte_slice(node.byte_range());
        tracing::debug!(
            "Getting completions for node at {}:{} with contents '{text}'",
            pos.line,
            pos.character
        );

        let mut items = Vec::new();
        items.extend(completion_for_keywords(&doc, pos, node));
        items.extend(completion_for_types(&doc, pos, node));
        items.extend(completion_for_instances(&doc, pos, node));
        items.extend(completion_for_options(&doc, pos, node).await);

        if items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(CompletionResponse::Array(
                items
                    .into_iter()
                    .map(|(kind, label)| CompletionItem {
                        kind: Some(kind),
                        label,
                        ..Default::default()
                    })
                    .collect(),
            )))
        }
    }

    async fn definition(
        &self,
        state: ServerState,
        params: GotoDefinitionParams,
    ) -> ServerResult<Option<GotoDefinitionResponse>> {
        let url = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let Some(doc) = state.document(&url) else {
            return Ok(None);
        };
        let Some(node) = doc.node_at_position_named(pos) else {
            tracing::debug!(
                "Missing node for definition at {}:{}",
                pos.line,
                pos.character
            );
            return Ok(None);
        };

        let text = doc.text().byte_slice(node.byte_range());
        tracing::debug!(
            "Getting definition for node at {}:{} with contents '{text}'",
            pos.line,
            pos.character
        );

        Ok(definition_for_types(&doc, pos, node))
    }
}
