#![allow(clippy::missing_panics_doc)]

use async_lsp_boilerplate::{
    Server, ServerResult, ServerState, Transport,
    lsp_types::{Hover, HoverContents, HoverParams, MarkedString},
    serve,
};

#[derive(Debug, Clone)]
struct ZapLanguageServer {}

impl Server for ZapLanguageServer {
    async fn hover(&self, _: ServerState, _: HoverParams) -> ServerResult<Option<Hover>> {
        Ok(Some(Hover {
            range: None,
            contents: HoverContents::Scalar(MarkedString::String(String::from(
                "Hello, zap language server!",
            ))),
        }))
    }
}

#[tokio::main]
pub async fn main() {
    let transport = Transport::Stdio;
    let server = ZapLanguageServer {};
    if let Err(e) = serve(transport, server).await {
        eprintln!("exiting due to fatal error\n{e}");
        std::process::exit(1)
    }
}
