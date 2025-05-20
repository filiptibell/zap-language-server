use std::fmt;

use tree_sitter::Node;

use crate::{format_node, result::Result, state::State, utils::is_type_empty};

pub(crate) fn format_struct(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    if is_type_empty(node) {
        write!(writer, "struct {{}}")?;
    } else {
        writeln!(writer, "struct {{")?;

        state.increase_depth();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "property" {
                format_struct_field(writer, state, child)?;
            }
        }

        state.decrease_depth();

        write!(writer, "{}}}", state.indent())?;
    }

    Ok(())
}

fn format_struct_field(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let key = node.child(0).expect("valid struct field");
    write!(writer, "{}{}: ", state.indent(), state.text(key))?;

    let typ = node.child(2).expect("valid struct field");
    format_node(writer, state, typ)?;

    writeln!(writer, ",")?;

    Ok(())
}
