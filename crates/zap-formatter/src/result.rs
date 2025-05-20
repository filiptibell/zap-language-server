use std::fmt;

/**
    An error that can occur while formatting a Zap document.
*/
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("encountered error node / invalid syntax at {0}:{1}")]
    Node(usize, usize),
    #[error("encountered internal error during formatting")]
    Fmt(#[from] fmt::Error),
}

/**
    Type alias for results that may return a Zap formatting error.
*/
pub type Result<T = ()> = std::result::Result<T, Error>;
