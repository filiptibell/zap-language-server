use async_lsp_boilerplate::{
    lsp_types::{
        ClientCapabilities, Hover, HoverContents, HoverParams, HoverProviderCapability,
        MarkedString, ServerCapabilities, ServerInfo, Url,
    },
    server::{Server, ServerResult, ServerState},
    tree_sitter::Language,
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
        let Some(doc) = state.document(&url) else {
            return Ok(None);
        };

        Ok(Some(Hover {
            range: None,
            contents: HoverContents::Scalar(MarkedString::String(String::from(
                if doc.has_syntax_tree() {
                    "Hello, zap language server! Syntax tree is available!"
                } else {
                    "Hello, zap language server! No syntax tree available :("
                },
            ))),
        }))
    }
}
