use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Failed to connect to port {0}")]
    TcpConnect(u16),
    #[error(transparent)]
    Lsp(#[from] async_lsp::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type ServerResult<T> = Result<T, ServerError>;
