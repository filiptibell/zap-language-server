use std::fmt;

use zap_language::{tree_sitter::Node, tree_sitter_utils::DepthFirstNodeIterator};

use crate::{result::Result, state::State};

pub(crate) fn format_plain(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let text = DepthFirstNodeIterator::new(node)
        .filter(|n| n.child_count() == 0)
        .map(|n| state.text(n))
        .collect::<Vec<_>>()
        .join(" ");

    write!(writer, "{text}")?;

    Ok(())
}
