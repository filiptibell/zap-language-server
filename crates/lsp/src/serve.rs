#![allow(clippy::missing_panics_doc)]

use std::num::NonZeroUsize;

use async_lsp::{
    ClientSocket, client_monitor::ClientProcessMonitorLayer, concurrency::ConcurrencyLayer,
    panic::CatchUnwindLayer, router::Router, server::LifecycleLayer, tracing::TracingLayer,
};
use tower::ServiceBuilder;

use crate::{
    result::ServerResult, server_trait::Server, server_with_documents::LanguageServerWithDocuments,
    transport::Transport,
};

/**
    Serves a language server over the given transport.

    This will automatically attach middleware for:

    - Tracing metadata for each request
    - Catching panics and safely returning internal server error statuses
    - Maximum concurrency of 8 in-flight LSP requests at a time
    - Client process monitoring and safe server shutdown

    # Errors

    - If the transport uses a socket and it could not connect
    - If the server encounters an I/O error while running
*/
pub async fn serve<F, S>(transport: Transport, make_server_fn: F) -> ServerResult<()>
where
    F: Fn(ClientSocket) -> S,
    S: Server,
{
    let (reader, writer) = transport.into_read_write().await?;

    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::new(NonZeroUsize::new(8).unwrap()))
            .layer(ClientProcessMonitorLayer::new(client.clone()))
            .service(Router::from_language_server(
                LanguageServerWithDocuments::new(make_server_fn(client)),
            ))
    });

    server
        .run_buffered(reader, writer)
        .await
        .map_err(Into::into)
}
