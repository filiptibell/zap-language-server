use std::fmt;

use tree_sitter::Node;

use crate::{format_unknown, result::Result, state::State};

mod enum_tagged;
mod enum_units;
mod structs;

use self::enum_tagged::format_enum_tagged;
use self::enum_units::format_enum_unit;
use self::structs::format_struct;

pub(crate) fn format_type(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    match node.kind() {
        "optional_type" => format_optional(writer, state, node),
        "struct_type" => format_struct(writer, state, node),
        "enum_unit_type" => format_enum_unit(writer, state, node),
        "enum_tagged_type" => format_enum_tagged(writer, state, node),
        "enum_type" => format_type(
            writer,
            state,
            node.child(0)
                .expect("base enum contains either unit or tagged type"),
        ),
        _ => format_unknown(writer, state, node),
    }
}

fn format_optional(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let typ = node.child(0).expect("valid optional type");
    format_type(writer, state, typ)?;
    write!(writer, "?")?;
    Ok(())
}
