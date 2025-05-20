use std::fmt;

use tree_sitter::Node;

use crate::{result::Result, state::State, utils::DepthFirstNodeIterator};

pub(crate) fn format_option_declaration(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    let text = DepthFirstNodeIterator::new(node)
        .filter(|n| n.child_count() == 0)
        .map(|n| state.text(n))
        .collect::<Vec<_>>()
        .join(" ");

    write!(writer, "{text}")?;

    Ok(())
}
