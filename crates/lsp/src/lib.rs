mod document;
mod result;
mod serve;
mod server_state;
mod server_trait;
mod server_with_state;
mod transport;

pub use async_lsp::lsp_types;

#[cfg(feature = "tree-sitter")]
pub use tree_sitter;

pub mod server {
    pub use crate::document::{Document, DocumentReader};
    pub use crate::result::{ServerError, ServerErrorCode, ServerResult};
    pub use crate::serve::serve;
    pub use crate::server_state::ServerState;
    pub use crate::server_trait::Server;
    pub use crate::transport::Transport;

    #[cfg(feature = "tree-sitter")]
    pub use crate::document::DocumentQueryCapture;
}
