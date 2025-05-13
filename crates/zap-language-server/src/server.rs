use async_language_server::{
    lsp_types::{
        ClientCapabilities, CompletionItem, CompletionOptions, CompletionParams,
        CompletionResponse, Hover, HoverParams, HoverProviderCapability, ServerCapabilities,
        ServerInfo, Url,
    },
    server::{Server, ServerResult, ServerState},
    tree_sitter::Language,
};

use crate::{
    completions::{
        completion_for_instances, completion_for_keywords, completion_for_options,
        completion_for_types, completion_trigger_characters,
    },
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

        let parent = node.parent();
        let parent = parent.as_ref();

        Ok(hover_for_keywords(&doc, &pos, &node, parent)
            .or_else(|| hover_for_types(&doc, &pos, &node, parent))
            .or_else(|| hover_for_properties(&doc, &pos, &node, parent))
            .or_else(|| hover_for_options(&doc, &pos, &node, parent)))
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

        let parent = node.parent();
        let parent = parent.as_ref();

        let mut items = Vec::new();
        items.extend(completion_for_keywords(&doc, &pos, &node, parent));
        items.extend(completion_for_types(&doc, &pos, &node, parent));
        items.extend(completion_for_instances(&doc, &pos, &node, parent));
        items.extend(completion_for_options(&doc, &pos, &node, parent).await);

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
}
