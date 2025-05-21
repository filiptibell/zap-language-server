use std::fmt;

use tree_sitter::Node;

use crate::{format_node, result::Result, state::State};

pub(crate) fn format_set(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let typ = node.child_by_field_name("type").expect("valid set");

    write!(writer, "set {{ ")?;
    format_node(writer, state, typ)?;
    write!(writer, " }}")?;

    Ok(())
}
