use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_contains_lsp_position,
};

use zap_language::docs::get_instance_class_names;

pub fn completion(
    doc: &Document,
    pos: Position,
    node: Node,
    parent: Option<Node>,
) -> Vec<(CompletionItemKind, String)> {
    let mut node = node;
    let mut parent = parent;

    // If our current node is the top-level "primitive type" we can
    // probably drill down to something a bit more specific & useful
    if node.kind() == "primitive_type" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" && ts_range_contains_lsp_position(child.range(), pos) {
                parent = Some(node);
                node = child;
                break;
            }
        }
    }

    let mut items = Vec::new();
    let Some(parent) = parent else { return items };

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
