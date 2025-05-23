use std::collections::HashSet;
use std::fmt;

use zap_language::{tree_sitter::Node, tree_sitter_utils::is_known_node};

use crate::{result::Result, state::State, utils::is_type_empty};

mod tagged;
mod untagged;

use self::tagged::{format_tagged_compact, format_tagged_multiline};
use self::untagged::{format_untagged_grid, format_untagged_line, format_untagged_multiline};

pub(crate) fn format_enum(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    let tag = node.child_by_field_name("tag").map(|t| state.text(t));
    if is_type_empty(node, Some(1)) {
        // No contents, single line with no space inbetween braces
        if let Some(tag) = tag {
            write!(writer, "enum {tag} {{}}")?;
        } else {
            write!(writer, "enum {{}}")?;
        }
    } else if tag.is_some() {
        // Tagged enum, format as one of these depending on what heuristic it matches:
        // - compact multiline, for uniform single-field variants
        // - generic multiline
        let mut variant_simple = true;
        let mut variant_len_min = usize::MAX;
        let mut variant_len_max = 0usize;
        let mut variant_field_names = HashSet::new();

        let mut cursor = node.walk();
        'outer: for child in node.children(&mut cursor).skip(2) {
            if child.kind() == "enum_variant" {
                let ident = child.child(0).expect("valid enum variant");
                let ident = state.text(ident);

                variant_len_min = variant_len_min.min(ident.len());
                variant_len_max = variant_len_max.max(ident.len());

                let mut child_cursor = child.walk();
                for descendant in child.children(&mut child_cursor).skip(2) {
                    if descendant.kind() == "property" {
                        let key = descendant.child(0).expect("valid enum variant field");
                        variant_field_names.insert(state.text(key));
                    } else if is_known_node(descendant) {
                        variant_simple = false;
                        break 'outer;
                    }
                }
            } else if is_known_node(child) {
                variant_simple = false;
                break 'outer;
            }
        }

        let format_as_compact = variant_simple // No unknown nodes that are not enum variants
            && (variant_len_max.saturating_sub(variant_len_min) <= 3) // Max variance of 3 chars in variant name length
            && variant_field_names.len() == 1; // All variants must have the same single field inside

        if format_as_compact {
            format_tagged_compact(writer, state, node, variant_len_max)?;
        } else {
            format_tagged_multiline(writer, state, node)?;
        }
    } else {
        // Untagged enum, format as one of these depending on what heuristic it matches:
        // - simple, few variants, single line
        // - uniform grid (3x3, 4x4, ...)
        // - generic multiline
        let mut cursor = node.walk();
        let mut identifiers = Vec::new();
        let mut all_children_are_variants = true;
        for child in node.children(&mut cursor).skip(1) {
            if child.kind() == "enum_variant" {
                let ident = child.child(0).expect("valid enum variant");
                identifiers.push(state.text(ident).to_string());
            } else if is_known_node(child) {
                all_children_are_variants = false;
                break;
            }
        }

        let mut format_as_line = false;
        let mut format_as_grid = false;

        if all_children_are_variants {
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
        }

        if format_as_line {
            format_untagged_line(writer, &identifiers)?;
        } else if format_as_grid {
            format_untagged_grid(writer, state, &identifiers)?;
        } else {
            format_untagged_multiline(writer, state, node)?;
        }
    }

    Ok(())
}
