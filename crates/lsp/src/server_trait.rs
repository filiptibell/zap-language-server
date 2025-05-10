use async_lsp::lsp_types::{ClientCapabilities, ServerCapabilities, ServerInfo};

#[allow(unused_variables)]
#[allow(clippy::must_use_candidate)]
pub trait Server {
    fn server_info() -> Option<ServerInfo> {
        None
    }

    fn server_capabilities(client_capabilities: ClientCapabilities) -> Option<ServerCapabilities> {
        None
    }
}
