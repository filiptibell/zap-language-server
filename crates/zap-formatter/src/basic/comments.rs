use std::fmt;

use tree_sitter::Node;

use crate::{result::Result, state::State};

pub(crate) fn format_top_level_comment(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    write!(writer, "{}", state.text(node))?; // No space before comment
    Ok(())
}

pub(crate) fn format_inline_comment(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    write!(writer, " {}", state.text(node))?; // Space before comment
    Ok(())
}
