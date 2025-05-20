use std::fmt;

use tree_sitter::Node;

mod basic;
mod config;
mod result;
mod state;
mod types;
mod utils;

use self::basic::{
    comments::{format_inline_comment, format_top_level_comment},
    declarations::format_declaration,
    options::format_option_declaration,
    unknown::format_unknown,
};
use self::state::State;
use self::types::format_type;
use self::utils::DepthFirstNodeIterator;

pub use self::config::{Config, Indentation};
pub use self::result::{Error, Result};

/**
    Formats a Zap document given a `config` and a tree-sitter `root` node.

    # Errors

    - If the given document tree contains any error node
    - If the formatter encounters an internal error / bug
*/
pub fn format_document(writer: &mut impl fmt::Write, config: Config, root: Node) -> Result {
    for node in DepthFirstNodeIterator::new(root) {
        if node.kind() == "ERROR" {
            let start = node.range().start_point;
            return Err(Error::Node(start.row, start.column));
        }
    }

    let mut state = State::new(config, 0);
    let mut cursor = root.walk();

    let mut last_end_row = 0;
    for child in root.children(&mut cursor) {
        let this_row_start = child.range().start_point.row;
        let this_row_end = child.range().end_point.row;

        let has_blank_line = last_end_row < this_row_start.saturating_sub(1);
        last_end_row = this_row_end;

        if has_blank_line {
            writeln!(writer)?;
        }

        if child.kind() == "comment" {
            format_top_level_comment(writer, &mut state, child)?;
        } else {
            format_node(writer, &mut state, child)?;
        }

        writeln!(writer)?;
    }

    Ok(())
}

fn format_node(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    match node.kind() {
        "comment" => format_inline_comment(writer, state, node),
        "option_declaration" => format_option_declaration(writer, state, node),
        "type_declaration" | "event_declaration" | "function_declaration" => {
            format_declaration(writer, state, node)
        }
        "optional_type" | "struct_type" | "enum_type" | "enum_unit_type" | "enum_tagged_type" => {
            format_type(writer, state, node)
        }
        _ => format_unknown(writer, state, node),
    }
}
