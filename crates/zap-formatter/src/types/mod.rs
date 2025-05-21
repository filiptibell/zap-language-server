use std::fmt;

use tree_sitter::Node;

use crate::basic::plain::format_plain;
use crate::{format_node, result::Result, state::State};

mod array;
mod enums;
mod range;
mod structs;

use self::array::format_array;
use self::enums::format_enum;
use self::range::format_range;
use self::structs::format_struct;

pub(crate) fn format_type(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    match node.kind() {
        "primitive_type" | "identifier" => format_plain(writer, state, node),
        "range" | "range_empty" | "range_exact" | "range_inexact" => {
            format_range(writer, state, node)
        }
        "array" | "array_empty" | "array_exact" | "array_inexact" => {
            format_array(writer, state, node)
        }
        "optional_type" => format_optional(writer, state, node),
        "struct_type" => format_struct(writer, state, node),
        "enum_type" => format_enum(writer, state, node),
        "type" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                format_node(writer, state, child)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn format_optional(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let typ = node.child(0).expect("valid optional type");
    format_type(writer, state, typ)?;
    write!(writer, "?")?;
    Ok(())
}
