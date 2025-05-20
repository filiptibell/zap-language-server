use std::fmt;

use tree_sitter::Node;

use crate::{format_node, result::Result, state::State, utils::is_type_empty};

pub(crate) fn format_enum_tagged(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    let tag = node.child(1).expect("valid tagged enum declaration");

    if is_type_empty(node) {
        write!(writer, "enum {} {{}}", state.text(tag))?;
    } else {
        writeln!(writer, "enum {} {{", state.text(tag))?;

        state.increase_depth();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "enum_tagged_variant" {
                format_enum_variant(writer, state, child)?;
            }
        }

        state.decrease_depth();

        write!(writer, "{}}}", state.indent())?;
    }

    Ok(())
}

fn format_enum_variant(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let variant = node.child(0).expect("valid enum variant");

    if is_type_empty(node) {
        write!(writer, "{}{} {{}}", state.indent(), state.text(variant))?;
    } else {
        writeln!(writer, "{}{} {{", state.indent(), state.text(variant))?;

        state.increase_depth();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "property" {
                format_enum_variant_field(writer, state, child)?;
            }
        }

        state.decrease_depth();

        writeln!(writer, "{}}},", state.indent())?;
    }

    Ok(())
}

fn format_enum_variant_field(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    let key = node.child(0).expect("valid enum variant field");
    write!(writer, "{}{}: ", state.indent(), state.text(key))?;

    let typ = node.child(2).expect("valid enum variant field");
    format_node(writer, state, typ)?;

    writeln!(writer, ",")?;

    Ok(())
}
