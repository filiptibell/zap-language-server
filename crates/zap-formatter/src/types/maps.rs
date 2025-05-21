use std::fmt;

use tree_sitter::Node;

use crate::{format_node, result::Result, state::State};

pub(crate) fn format_map(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let key = node.child_by_field_name("key_type").expect("valid map");
    let val = node.child_by_field_name("value_type").expect("valid map");

    write!(writer, "map {{ [{}]: ", state.text(key))?;
    format_node(writer, state, val)?;
    write!(writer, " }}")?;

    Ok(())
}
