use std::fmt;

use tree_sitter::Node;

use crate::{format_node, result::Result, state::State, utils::is_type_empty};

pub(crate) fn format_enum(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let tag = node.child_by_field_name("tag").map(|t| state.text(t));
    if is_type_empty(node) {
        // No contents, single line with no space inbetween braces
        if let Some(tag) = tag {
            write!(writer, "enum {tag} {{}}")?;
        } else {
            write!(writer, "enum {{}}")?;
        }
    } else if let Some(tag) = tag {
        // Tagged enum, format as multiline
        writeln!(writer, "enum {tag} {{")?;

        state.increase_depth();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "enum_variant" {
                format_tagged_variant(writer, state, child)?;
            }
        }

        state.decrease_depth();

        write!(writer, "{}}}", state.indent())?;
    } else {
        // Untagged enum, format as one of: singe line, grid (3x3, 4x4, etc), multiline
        let mut cursor = node.walk();
        let mut identifiers = Vec::new();
        for child in node.children(&mut cursor) {
            if child.kind() == "enum_variant" {
                let ident = child.child(0).expect("valid enum variant");
                identifiers.push(state.text(ident).to_string());
            }
        }

        let mut format_as_line = false;
        let mut format_as_grid = false;

        if identifiers.len() <= 4 {
            let total_chars = 2 // Braces
        + identifiers.len() // Commas
        + identifiers.len().saturating_sub(1) // Spacing
        + identifiers.iter().map(String::len).sum::<usize>();
            if total_chars <= state.config.columns {
                format_as_line = true;
            }
        } else if matches!(identifiers.len(), 9 | 16 | 25 | 36) {
            if let Some(first_len) = identifiers.first().map(String::len) {
                if identifiers.iter().all(|i| i.len() == first_len) {
                    format_as_grid = true;
                }
            }
        }

        if format_as_line {
            format_untagged_line(writer, &identifiers)?;
        } else if format_as_grid {
            format_untagged_grid(writer, state, &identifiers)?;
        } else {
            format_untagged_multiline(writer, state, &identifiers)?;
        }
    }

    Ok(())
}

fn format_tagged_variant(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let variant = node.child(0).expect("valid enum variant");

    if is_type_empty(node) {
        write!(writer, "{}{} {{}}", state.indent(), state.text(variant))?;
    } else {
        writeln!(writer, "{}{} {{", state.indent(), state.text(variant))?;

        state.increase_depth();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "property" {
                let key = child.child(0).expect("valid enum variant field");
                write!(writer, "{}{}: ", state.indent(), state.text(key))?;

                let typ = child.child(2).expect("valid enum variant field");
                format_node(writer, state, typ)?;

                writeln!(writer, ",")?;
            }
        }

        state.decrease_depth();

        writeln!(writer, "{}}},", state.indent())?;
    }

    Ok(())
}

/**
    Formats an untagged enum as a single line.

    Example:

    ```zap
    type SingleLineEnum = enum { One, Two, Three }
    ```
*/
fn format_untagged_line(writer: &mut impl fmt::Write, identifiers: &[String]) -> Result {
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
fn format_untagged_grid(
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
        Four,
        Five,
        Six,
    }
    ```
*/
fn format_untagged_multiline(
    writer: &mut impl fmt::Write,
    state: &mut State,
    identifiers: &[String],
) -> Result {
    writeln!(writer, "enum {{")?;

    state.increase_depth();

    for identifier in identifiers {
        writeln!(writer, "{}{identifier},", state.indent())?;
    }

    state.decrease_depth();

    write!(writer, "{}}}", state.indent())?;

    Ok(())
}
