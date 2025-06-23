use std::fmt;

use zap_language::{tree_sitter::Node, tree_sitter_utils::is_known_node};

use crate::{format_node, format_plain, result::Result, state::State, types::format_type};

pub(crate) fn format_declaration(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    format_declaration_pre(writer, state, node)?;

    if node.kind() == "option_declaration" {
        let value = node.child(3).expect("valid option declaration");
        let value = state.text(value);
        write!(writer, "{value}")?;
    } else if node.kind() == "type_declaration" {
        if let Some(value) = node.child(3) {
            format_type(writer, state, value)?;
        }
    } else if matches!(node.kind(), "event_declaration" | "function_declaration") {
        writeln!(writer, "{{")?;

        state.increase_depth();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor).skip(2) {
            match child.kind() {
                "event_from_field"
                | "event_type_field"
                | "event_call_field"
                | "event_data_field"
                | "function_call_field"
                | "function_args_field"
                | "function_rets_field" => {
                    write!(writer, "{}", state.indent())?;
                    format_declaration_field(writer, state, child)?;
                }
                _ if is_known_node(child) => {
                    write!(writer, "{}", state.indent())?;
                    format_node(writer, state, child)?;
                }
                _ => {}
            }
        }

        state.decrease_depth();

        write!(writer, "{}}}", state.indent())?;
    } else if node.kind() == "namespace_declaration" {
        writeln!(writer, "{{")?;

        state.increase_depth();

        let mut cursor = node.walk();
        let mut last_end_row = node
            .child(3)
            .map(|n| n.range().end_point.row)
            .unwrap_or_default();
        for child in node.children(&mut cursor).skip(3) {
            if matches!(child.kind(), "=" | "{" | "}") {
                continue;
            }

            let this_row_start = child.range().start_point.row;
            let this_row_end = child.range().end_point.row;

            let has_blank_line = last_end_row < this_row_start.saturating_sub(1);
            last_end_row = this_row_end;

            if has_blank_line {
                writeln!(writer)?;
            }

            write!(writer, "{}", state.indent())?;
            format_node(writer, state, child)?;
            writeln!(writer)?;
        }

        // NOTE: We should preserve a single empty line before the closing
        // brace if a user has added one, for consistency, since we also
        // preserve a single opening empty line in our main loop above
        if let Some(end_bracket) = node.child(node.child_count() - 1) {
            if end_bracket.kind() == "}" {
                let end_bracket_start = end_bracket.range().start_point.row;
                let has_blank_line = last_end_row < end_bracket_start.saturating_sub(1);
                if has_blank_line {
                    writeln!(writer)?;
                }
            }
        }

        state.decrease_depth();

        write!(writer, "{}}}", state.indent())?;
    }

    Ok(())
}

fn format_declaration_pre(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let keyword = node.child(0).expect("valid declaration");
    let identifier = node.child(1).expect("valid declaration");

    write!(
        writer,
        "{} {} = ",
        state.text(keyword),
        state.text(identifier)
    )?;

    Ok(())
}

fn format_declaration_field(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let key = node
        .kind()
        .trim_start_matches("event_")
        .trim_start_matches("function_")
        .trim_end_matches("_field");
    write!(writer, "{key}: ")?;

    let value = node.child(2).expect("valid event or function field");
    if is_known_node(value) {
        format_node(writer, state, value)?;
    } else {
        format_plain(writer, state, value)?;
    }

    writeln!(writer, ",")?;

    Ok(())
}
