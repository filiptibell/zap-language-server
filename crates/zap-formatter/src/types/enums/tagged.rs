use std::fmt;

use zap_language::{
    tree_sitter::Node,
    tree_sitter_utils::{is_comment_node, is_known_node},
};

use crate::{format_node, result::Result, state::State, utils::is_type_empty};

/**
    Formats a tagged enum using compact formatting.

    Example:

    ```zap
    type CompactTagged = enum "Kind" {
        AA { Value: u8 },
        BB { Value: u16 },
        CC { Value: u32 },
    }
    ```
*/
pub(super) fn format_tagged_compact(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
    variant_len_max: usize,
) -> Result {
    let tag = node.child_by_field_name("tag").unwrap();
    let tag = state.text(tag);

    writeln!(writer, "enum {tag} {{")?;

    state.increase_depth();

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "enum_variant" {
            write!(writer, "{}", state.indent())?;
            format_variant_compact(writer, state, child, variant_len_max)?;
            writeln!(writer, ",")?;
        } else if is_known_node(child) {
            unreachable!("not a compact tagged enum (??)")
        }
    }

    state.decrease_depth();

    write!(writer, "{}}}", state.indent())?;

    Ok(())
}

fn format_variant_compact(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
    variant_len_max: usize,
) -> Result {
    let ident = node.child(0).expect("valid enum variant");
    let ident = state.text(ident);

    let spaces = " ".repeat(variant_len_max.saturating_sub(ident.len()));

    let mut field = None;
    let mut cursor = node.walk();
    for descendant in node.children(&mut cursor) {
        if descendant.kind() == "property" {
            field.replace(descendant);
            break;
        }
    }

    if let Some(field) = field {
        let field_key = state.text(field.child(0).expect("valid tagged enum field"));
        let field_typ = state.text(field.child(2).expect("valid tagged enum field"));

        write!(writer, "{ident}{spaces} {{ {field_key}: {field_typ} }}")?;
    } else {
        write!(writer, "{ident}{spaces} {{}}")?;
    }

    Ok(())
}

/**
    Formats a tagged enum using regular multiline formatting.

    Example:

    ```zap
    type Tagged = enum "Kind" {
        VariantEmpty {},
        VariantOne {
            Foo: u8,
        },
        VariantTwo {
            Bar: u16,
        },
        VariantThree {
            Baz: u32,
        },
    }
    ```
*/
pub(super) fn format_tagged_multiline(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    let tag = node.child_by_field_name("tag").unwrap();
    let tag = state.text(tag);

    writeln!(writer, "enum {tag} {{")?;

    state.increase_depth();

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "enum_variant" {
            write!(writer, "{}", state.indent())?;
            format_variant_multiline(writer, state, child)?;
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

    Ok(())
}

fn format_variant_multiline(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let ident = node.child(0).expect("valid enum variant");
    let ident = state.text(ident);

    if is_type_empty(node, Some(1)) {
        write!(writer, "{ident} {{}}")?;
    } else {
        writeln!(writer, "{ident} {{")?;

        state.increase_depth();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor).skip(2) {
            if child.kind() == "property" {
                let key = child.child(0).expect("valid enum variant field");
                let typ = child.child(2).expect("valid enum variant field");

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
