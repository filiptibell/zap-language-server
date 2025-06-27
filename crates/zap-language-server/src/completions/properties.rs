use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{find_child, find_descendant, ts_range_contains_lsp_position},
};
use zap_language::{
    docs::{find_variants, get_property_names},
    tree_sitter_utils::is_field_node,
};

use crate::utils::is_namespace;

pub fn completion(_doc: &Document, pos: Position, node: Node) -> Vec<(CompletionItemKind, String)> {
    // If our current node is a top-level, we can probably
    // find something that is a bit more specific & useful
    let node = if is_namespace(node) {
        find_child(node, |c| {
            ts_range_contains_lsp_position(c.range(), pos) && is_decl_node(c)
        })
        .unwrap_or(node)
    } else {
        node
    };

    // Try to find the field node, and then the value node inside of it
    let node = find_child(node, |d| {
        ts_range_contains_lsp_position(d.range(), pos) && is_field_node(d)
    })
    .unwrap_or(node);
    let node = node.child_by_field_name("value").unwrap_or(node);

    // At this point we are either in a proper declaration field,
    // a generic property node, or an incomplete error node
    let mut items = Vec::new();

    // If we are inside an unfinished "error" node, add property name completions
    if is_decl_node(node)
        && find_descendant(node, |e| e.is_error())
            .is_some_and(|d| ts_range_contains_lsp_position(d.range(), pos))
    {
        items.extend(
            get_property_names().map(|prop| (CompletionItemKind::PROPERTY, prop.to_string())),
        );
    }

    // If we are inside the value node, and have field as parent, complete enums
    if let Some(parent) = node.parent().filter(|n| is_field_node(*n)) {
        if let Some((false, variants)) = find_variants([parent.kind()]) {
            items.extend(
                variants
                    .iter()
                    .map(|word| (CompletionItemKind::ENUM_MEMBER, (*word).to_string())),
            );
        }
    }

    items
}

fn is_decl_node(node: Node) -> bool {
    matches!(node.kind(), "event_declaration" | "function_declaration")
}
