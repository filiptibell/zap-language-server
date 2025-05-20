use anyhow::Result;
use clap::{Parser, Subcommand};

mod fmt;
mod serve;

use self::fmt::FormatCommand;
use self::serve::ServeCommand;

#[derive(Debug, Clone, Subcommand)]
pub enum CliSubcommand {
    Fmt(FormatCommand),
    Serve(ServeCommand),
}

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    subcommand: CliSubcommand,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub async fn run(self) -> Result<()> {
        match self.subcommand {
            CliSubcommand::Fmt(cmd) => cmd.run().await,
            CliSubcommand::Serve(cmd) => cmd.run().await,
        }
    }
}
