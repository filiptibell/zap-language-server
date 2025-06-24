mod cli;
mod completions;
mod definitions;
mod hovers;
mod references;
mod renames;
mod server;
mod structs;
mod tracing;
mod utils;

use self::tracing::setup_tracing;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    setup_tracing();
    cli::Cli::new().run().await
}
