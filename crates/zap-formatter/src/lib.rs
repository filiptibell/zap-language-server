use std::fmt;

use zap_language::tree_sitter::Node;

mod basic;
mod config;
mod result;
mod state;
mod types;

use self::basic::{
    comments::format_comment, declarations::format_declaration, plain::format_plain,
};
use self::state::State;
use self::types::format_type;

pub use self::config::{Config, Indentation};
pub use self::result::{Error, Result};

/**
    Formats a single node given a `config` and a tree-sitter `root` node.

    This has slightly different behavior compared to `format_document` in terms of
    whitespace and newlines - prefer `format_document` for full document formatting.

    Designed to be used for temporary formatting, such as in type hovers.

    # Errors

    - If the given document tree contains any error node
    - If the formatter encounters an internal error / bug
*/
pub fn format_root(writer: &mut impl fmt::Write, config: Config, root: Node) -> Result {
    let mut state = State::new(config, 0);
    format_node(writer, &mut state, root)?;
    Ok(())
}

/**
    Formats a Zap document given a `config` and a tree-sitter `root` node.

    # Errors

    - If the given document tree contains any error node
    - If the formatter encounters an internal error / bug
*/
pub fn format_document(writer: &mut impl fmt::Write, config: Config, root: Node) -> Result {
    use zap_language::tree_sitter_utils::DepthFirstNodeIterator;

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
            format_comment(writer, &mut state, child)?;
        } else {
            format_node(writer, &mut state, child)?;
        }

        writeln!(writer)?;
    }

    Ok(())
}

fn format_node(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    use zap_language::tree_sitter_utils::{
        is_array_node, is_comment_node, is_declaration_node, is_ident_node, is_range_node,
        is_tuple_node, is_type_node,
    };

    if is_comment_node(node) {
        format_comment(writer, state, node)?;
    } else if is_declaration_node(node) {
        format_declaration(writer, state, node)?;
    } else if is_type_node(node)
        || is_tuple_node(node)
        || is_range_node(node)
        || is_array_node(node)
    {
        format_type(writer, state, node)?;
    } else if is_ident_node(node) {
        format_plain(writer, state, node)?;
    }

    Ok(())
}
