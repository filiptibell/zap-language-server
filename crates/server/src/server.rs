use async_lsp_boilerplate::{
    lsp_types::{
        ClientCapabilities, Hover, HoverContents, HoverParams, HoverProviderCapability,
        MarkedString, ServerCapabilities, ServerInfo, Url,
    },
    server::{Server, ServerResult, ServerState},
    tree_sitter::Language,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::docs::{find_docs_enum, find_docs_option};

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
            ..Default::default()
        })
    }

    fn determine_tree_sitter_language(_: &Url, language: &str) -> Option<Language> {
        if language.eq_ignore_ascii_case("zap") {
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

        if let Some((head, desc)) = find_docs_enum([parent.kind(), node.kind()]).or_else(|| {
            if parent.kind() == "option_declaration" && node.kind() == "identifier" {
                let ident = doc.text().byte_slice(node.byte_range());
                find_docs_option([ident])
            } else {
                None
            }
        }) {
            return Ok(Some(Hover {
                range: Some(ts_range_to_lsp_range(node.range())),
                contents: HoverContents::Scalar(MarkedString::String(format!(
                    "# {head}\n\n{desc}\n"
                ))),
            }));
        }

        Ok(None)

        // Ok(Some(Hover {
        //     range: Some(ts_range_to_lsp_range(node.range())),
        //     contents: HoverContents::Scalar(MarkedString::String(format!(
        //         "# {} > {}\n\n{}",
        //         parent.kind(),
        //         node.kind(),
        //         doc.text().byte_slice(node.byte_range()).to_string(),
        //     ))),
        // }))
    }
}
