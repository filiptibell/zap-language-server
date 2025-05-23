use async_language_server::{
    lsp_types::{
        ClientCapabilities, CompletionItem, CompletionOptions, CompletionParams,
        CompletionResponse, DocumentFormattingParams, GotoDefinitionParams, GotoDefinitionResponse,
        Hover, HoverParams, HoverProviderCapability, OneOf, PrepareRenameResponse, RenameOptions,
        RenameParams, ServerCapabilities, ServerInfo, TextDocumentPositionParams, TextEdit,
        WorkDoneProgressOptions, WorkspaceEdit,
    },
    server::{DocumentMatcher, Server, ServerError, ServerResult, ServerState},
    tree_sitter_utils::ts_range_to_lsp_range,
};
use zap_formatter::Config;

use crate::{
    completions::{
        completion_for_instances, completion_for_keywords, completion_for_options,
        completion_for_properties, completion_for_types, completion_trigger_characters,
    },
    definitions::definition_for_types,
    hovers::{hover_for_keywords, hover_for_options, hover_for_properties, hover_for_types},
    renames::{rename_for_types, rename_prepare_for_types},
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
            rename_provider: Some(OneOf::Right(RenameOptions {
                prepare_provider: Some(true),
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: None,
                },
            })),
            definition_provider: Some(OneOf::Left(true)),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(true),
                trigger_characters: Some(completion_trigger_characters()),
                ..Default::default()
            }),
            document_formatting_provider: Some(OneOf::Left(true)),
            ..Default::default()
        })
    }

    fn server_document_matchers() -> Vec<DocumentMatcher> {
        vec![
            DocumentMatcher::new("Zap Document")
                .with_url_globs(["*.zap"])
                .with_lang_strings(["Zap"])
                .with_lang_grammar(zap_language::TS_LANGUAGE.into()),
        ]
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

        tracing::debug!("Getting hover for node at {}:{}", pos.line, pos.character);

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

        tracing::debug!(
            "Getting completions for node at {}:{}",
            pos.line,
            pos.character
        );

        let mut items = Vec::new();
        items.extend(completion_for_keywords(&doc, pos, node));
        items.extend(completion_for_types(&doc, pos, node));
        items.extend(completion_for_instances(&doc, pos, node));
        items.extend(completion_for_properties(&doc, pos, node));
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

    async fn rename_prepare(
        &self,
        state: ServerState,
        params: TextDocumentPositionParams,
    ) -> ServerResult<Option<PrepareRenameResponse>> {
        let url = params.text_document.uri;
        let pos = params.position;

        let Some(doc) = state.document(&url) else {
            return Ok(None);
        };
        let Some(node) = doc.node_at_position_named(pos) else {
            tracing::debug!("Missing node for rename at {}:{}", pos.line, pos.character);
            return Ok(None);
        };

        Ok(rename_prepare_for_types(&doc, pos, node))
    }

    async fn rename(
        &self,
        state: ServerState,
        params: RenameParams,
    ) -> ServerResult<Option<WorkspaceEdit>> {
        let url = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        let Some(doc) = state.document(&url) else {
            return Ok(None);
        };
        let Some(node) = doc.node_at_position_named(pos) else {
            tracing::debug!("Missing node for rename at {}:{}", pos.line, pos.character);
            return Ok(None);
        };

        Ok(rename_for_types(&doc, pos, node, params.new_name.as_str()))
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

        if doc.node_at_root().is_some_and(|r| node == r) {
            return Ok(None); // Cant go to definition on the root node
        }

        tracing::debug!(
            "Getting definition for node at {}:{}",
            pos.line,
            pos.character
        );

        Ok(definition_for_types(&doc, pos, node))
    }

    async fn document_format(
        &self,
        state: ServerState,
        params: DocumentFormattingParams,
    ) -> ServerResult<Option<Vec<TextEdit>>> {
        let url = params.text_document.uri;

        let Some(doc) = state.document(&url) else {
            return Ok(None);
        };
        let Some(root) = doc.node_at_root() else {
            return Ok(None);
        };

        let text = doc.text_bytes();
        let config = Config::new(text.as_slice());

        let mut new_text = String::new();
        if let Err(e) = zap_formatter::format_document(&mut new_text, config, root) {
            return Err(ServerError::unknown(e));
        }

        let range = ts_range_to_lsp_range(root.range());
        Ok(Some(vec![TextEdit { range, new_text }]))
    }
}
