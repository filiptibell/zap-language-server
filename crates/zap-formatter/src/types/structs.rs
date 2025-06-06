use std::fmt;

use zap_language::{
    tree_sitter::Node,
    tree_sitter_utils::{is_comment_node, is_known_node, is_type_empty},
};

use crate::{format_node, result::Result, state::State};

pub(crate) fn format_struct(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    if is_type_empty(node, None) {
        write!(writer, "struct {{}}")?;
    } else {
        writeln!(writer, "struct {{")?;

        state.increase_depth();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "property" {
                let key = child.child(0).expect("valid struct field");
                let typ = child.child(2).expect("valid struct field");

                write!(writer, "{}{}: ", state.indent(), state.text(key))?;
                format_node(writer, state, typ)?;
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

        write!(writer, "{}}}", state.indent())?;
    }

    Ok(())
}
