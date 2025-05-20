use std::fmt;

use tree_sitter::Node;

use crate::{format_unknown, result::Result, state::State};

mod enums;
mod structs;

use self::enums::format_enum;
use self::structs::format_struct;

pub(crate) fn format_type(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    match node.kind() {
        "optional_type" => format_optional(writer, state, node),
        "struct_type" => format_struct(writer, state, node),
        "enum_type" => format_enum(writer, state, node),
        _ => format_unknown(writer, state, node),
    }
}

fn format_optional(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let typ = node.child(0).expect("valid optional type");
    format_type(writer, state, typ)?;
    write!(writer, "?")?;
    Ok(())
}
