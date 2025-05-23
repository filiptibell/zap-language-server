use std::fmt;

use zap_language::{
    tree_sitter::Node,
    tree_sitter_utils::{is_comment_node, is_known_node, is_type_empty},
};

use crate::{format_node, result::Result, state::State};

pub(crate) fn format_tuple(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    if is_type_empty(node, None) {
        write!(writer, "()")?;
    } else {
        writeln!(writer, "(")?;

        state.increase_depth();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "tuple_value" {
                let typ = child
                    .child_by_field_name("type")
                    .expect("valid tuple value");

                if let Some(name) = child.child_by_field_name("name") {
                    write!(writer, "{}{}: ", state.indent(), state.text(name))?;
                    format_node(writer, state, typ)?;
                } else {
                    write!(writer, "{}", state.indent())?;
                    format_node(writer, state, typ)?;
                }

                writeln!(writer, ",")?;
            } else if is_known_node(child) {
                write!(writer, "{}", state.indent())?;
                format_node(writer, state, child)?;

                if is_comment_node(child) {
                    writeln!(writer)?;
                } else {
                    writeln!(writer, ",")?;
                }
            }
        }

        state.decrease_depth();

        write!(writer, "{})", state.indent())?;
    }

    Ok(())
}
