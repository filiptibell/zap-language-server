use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{find_child, ts_range_contains_lsp_position},
};
use zap_language::{docs::find_variants, tree_sitter_utils::is_field_node};

use crate::utils::is_namespace;

pub fn completion(_doc: &Document, pos: Position, node: Node) -> Vec<(CompletionItemKind, String)> {
    // If our current node is a top-level, we can probably
    // find something that is a bit more specific & useful
    let node = if is_namespace(node) {
        find_child(node, |c| {
            ts_range_contains_lsp_position(c.range(), pos)
                && matches!(c.kind(), "event_declaration" | "function_declaration")
        })
        .unwrap_or(node)
    } else {
        node
    };

    // Try to find the field node
    let node = find_child(node, |d| {
        ts_range_contains_lsp_position(d.range(), pos) && is_field_node(d)
    })
    .unwrap_or(node);

    // Try to find the value node
    let node = node.child_by_field_name("value").unwrap_or(node);

    // We should now be inside the value node, and have field as parent
    let Some(parent) = node.parent().filter(|n| is_field_node(*n)) else {
        return Vec::new();
    };

    let mut items = Vec::new();

    if let Some((false, variants)) = find_variants([parent.kind()]) {
        items.extend(
            variants
                .iter()
                .map(|word| (CompletionItemKind::ENUM_MEMBER, (*word).to_string())),
        );
    }

    items
}
