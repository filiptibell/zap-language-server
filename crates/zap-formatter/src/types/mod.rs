use std::fmt;

use zap_language::tree_sitter::Node;

use crate::basic::plain::format_plain;
use crate::{format_node, result::Result, state::State};

mod arrays;
mod enums;
mod maps;
mod primitives;
mod ranges;
mod sets;
mod structs;
mod tuples;

use self::arrays::format_array;
use self::enums::format_enum;
use self::maps::format_map;
use self::primitives::format_primitive;
use self::ranges::format_range;
use self::sets::format_set;
use self::structs::format_struct;
use self::tuples::format_tuple;

pub(crate) fn format_type(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    match node.kind() {
        "identifier" => format_plain(writer, state, node),

        "range" | "range_empty" | "range_exact" | "range_inexact" => {
            format_range(writer, state, node)
        }
        "array" | "array_empty" | "array_exact" | "array_inexact" => {
            format_array(writer, state, node)
        }

        "primitive_type" => format_primitive(writer, state, node),
        "namespaced_type" => format_namespaced(writer, state, node),
        "optional_type" => format_optional(writer, state, node),
        "struct_type" => format_struct(writer, state, node),
        "enum_type" => format_enum(writer, state, node),
        "set_type" => format_set(writer, state, node),
        "map_type" => format_map(writer, state, node),
        "tuple" => format_tuple(writer, state, node),

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

fn format_namespaced(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let mut cursor = node.walk();
    for namespace in node.children_by_field_name("namespace", &mut cursor) {
        format_node(writer, state, namespace)?;
        write!(writer, ".")?;
    }

    let typ = node
        .child_by_field_name("type")
        .expect("valid namespaced type");
    format_type(writer, state, typ)?;

    Ok(())
}

fn format_optional(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let typ = node.child(0).expect("valid optional type");
    format_type(writer, state, typ)?;
    write!(writer, "?")?;
    Ok(())
}
