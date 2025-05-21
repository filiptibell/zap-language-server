use std::fmt;

use tree_sitter::Node;

use crate::{format_node, is_comment_node, is_known_node, result::Result, state::State};

/**
    Formats an untagged enum as a single line.

    Example:

    ```zap
    type SingleLineEnum = enum { One, Two, Three }
    ```
*/
pub(super) fn format_untagged_line(writer: &mut impl fmt::Write, identifiers: &[String]) -> Result {
    write!(writer, "enum {{ ")?;

    for (index, identifier) in identifiers.iter().enumerate() {
        if index != 0 {
            write!(writer, ", ")?;
        }
        write!(writer, "{identifier}")?;
    }

    write!(writer, " }}")?;

    Ok(())
}

/**
    Formats an untagged enum evenly in a grid pattern, over multiple lines.

    Supports even grids in sizes: 3x3, 4x4, 5x5, 6x6

    Example:

    ```zap
    type Grid3x3 = enum {
        AA, AB, AC,
        BA, BB, BC,
        CA, CB, CC,
    }

    type Grid4x4 = enum {
        AA, AB, AC, AD,
        BA, BB, BC, BD,
        CA, CB, CC, CD,
        DA, DB, DC, DD,
    }
    ```
*/
pub(super) fn format_untagged_grid(
    writer: &mut impl fmt::Write,
    state: &mut State,
    identifiers: &[String],
) -> Result {
    writeln!(writer, "enum {{")?;

    state.increase_depth();

    let chunk = match identifiers.len() {
        9 => 3,
        16 => 4,
        25 => 5,
        36 => 6,
        _ => unreachable!(),
    };

    for identifier_chunk in identifiers.chunks_exact(chunk) {
        write!(writer, "{}", state.indent())?;
        for (index, identifier) in identifier_chunk.iter().enumerate() {
            if index != 0 {
                write!(writer, ", ")?;
            }
            write!(writer, "{identifier}")?;
        }
        writeln!(writer, ",")?;
    }

    state.decrease_depth();

    write!(writer, "{}}}", state.indent())?;

    Ok(())
}

/**
    Formats an untagged enum over multiple lines with a single item per line.

    Example:

    ```zap
    type MultilineEnum = enum {
        One,
        Two,
        Three,
        -- Maybe a comment
        Four,
        Five,
        Six,
    }
    ```
*/
pub(super) fn format_untagged_multiline(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    writeln!(writer, "enum {{")?;

    state.increase_depth();

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "enum_variant" {
            let ident = child.child(0).expect("valid enum variant");
            writeln!(writer, "{}{},", state.indent(), state.text(ident))?;
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

    Ok(())
}
