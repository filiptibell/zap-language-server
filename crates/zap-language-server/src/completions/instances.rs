use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{find_child, ts_range_contains_lsp_position},
};

use zap_language::docs::get_instance_class_names;

pub fn completion(doc: &Document, pos: Position, node: Node) -> Vec<(CompletionItemKind, String)> {
    // If our current node is the top-level "primitive type" we can
    // probably drill down to something a bit more specific & useful
    let node = if node.kind() == "primitive_type" {
        find_child(node, |c| {
            let is_ident = c.kind() == "identifier";
            let is_inside = ts_range_contains_lsp_position(c.range(), pos);
            is_ident && is_inside
        })
        .unwrap_or(node)
    } else {
        node
    };

    let mut items = Vec::new();
    let Some(parent) = node.parent() else {
        return items;
    };

    // Make sure that we are actually inside the range
    // clause for an Instance primitive type, specifically
    if parent.kind() == "primitive_type" && node.kind() == "identifier" {
        if let Some(first_child) = parent.child(0) {
            let first_text = doc.text().byte_slice(first_child.byte_range());
            if first_text.as_str().is_some_and(|t| t == "Instance") {
                items.extend(
                    get_instance_class_names()
                        .map(|name| (CompletionItemKind::VALUE, name.to_string())),
                );
            }
        }
    }

    items
}
