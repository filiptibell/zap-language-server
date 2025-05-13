mod cli;
mod completions;
mod definitions;
mod hovers;
mod server;
mod tracing;
mod utils;

use self::tracing::setup_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing();
    cli::Cli::new().run().await
}
