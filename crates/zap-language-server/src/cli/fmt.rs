use std::{ffi::OsStr, path::PathBuf, process::exit};

use anyhow::{Context, Result};
use clap::{Parser, ValueHint};
use fs_err::tokio as fs;
use tokio::io::{AsyncReadExt, stdin};

use console::{Style, style};
use similar::{ChangeTag, TextDiff};

use async_language_server::tree_sitter;
use zap_formatter::Config;

#[derive(Debug, Clone, Parser)]
pub struct FormatCommand {
    #[arg(value_hint = ValueHint::FilePath)]
    pub file: PathBuf,
    #[arg(long)]
    pub check: bool,
    #[arg(long)]
    pub write: bool,
}

impl FormatCommand {
    pub async fn run(self) -> Result<()> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_zap::LANGUAGE.into())
            .context("Failed to set parser language")?;

        let use_stdin = self.file.file_name().is_some_and(|n| n == OsStr::new("-"));
        let source = if use_stdin {
            let mut s = String::new();
            stdin()
                .read_to_string(&mut s)
                .await
                .context("Failed to read stdin input")?;
            s
        } else {
            fs::read_to_string(&self.file)
                .await
                .context("Failed to read input file")?
        };

        let tree = parser
            .parse(&source, None)
            .context("Failed to parse input file")?;

        let config = Config::new(source.as_bytes());

        let mut formatted = String::new();
        zap_formatter::format_document(&mut formatted, config, tree.root_node())
            .context("Failed to write formatted result")?;

        if self.check {
            let diff = TextDiff::from_lines(source.as_str(), formatted.as_str());

            let mut any_change = false;
            for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
                any_change = true;

                if idx > 0 {
                    println!("\n{}\n", "-".repeat(80));
                } else {
                    println!();
                }

                for op in group {
                    for change in diff.iter_changes(op) {
                        let (sign, s) = match change.tag() {
                            ChangeTag::Delete => ("-", Style::new().red()),
                            ChangeTag::Insert => ("+", Style::new().green()),
                            ChangeTag::Equal => (" ", Style::new().dim()),
                        };

                        print!(
                            "{}{} |{}{}",
                            style(format_line_opt(change.old_index())).dim(),
                            style(format_line_opt(change.new_index())).dim(),
                            s.apply_to(sign).bold(),
                            s.apply_to(change.to_string_lossy()),
                        );

                        if change.missing_newline() {
                            println!();
                        }
                    }
                }
            }

            if any_change {
                println!();
                exit(1);
            }
        } else if self.write && !use_stdin {
            fs::write(self.file, formatted)
                .await
                .context("Failed to write output file")?;
        } else {
            print!("{formatted}");
        }

        Ok(())
    }
}

fn format_line_opt(line: Option<usize>) -> String {
    match line {
        None => String::from("    "),
        Some(l) => format!("{:<4}", l + 1),
    }
}
