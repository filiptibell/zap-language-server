use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{find_child, find_descendant, ts_range_contains_lsp_position},
};

use crate::{
    structs::ReferencedType,
    utils::{is_namespace, is_type},
};

pub fn completion(doc: &Document, pos: Position, node: Node) -> Vec<(CompletionItemKind, String)> {
    // If our current node is a top-level, we can probably
    // find something that is a bit more specific & useful
    let node = if is_namespace(node) {
        find_child(node, |c| {
            ts_range_contains_lsp_position(c.range(), pos) && c.kind() == "type_declaration"
        })
        .unwrap_or(node)
    } else {
        node
    };

    // Try to find an even more specific node, if possible
    let node = find_descendant(node, |d| {
        ts_range_contains_lsp_position(d.range(), pos) && is_type(d)
    })
    .unwrap_or(node);

    let mut items = Vec::new();

    if let Some(typ) = node.parent().and_then(ReferencedType::from_node) {
        if ts_range_contains_lsp_position(typ.identifier_range(), pos) {
            // Node is C in a namespaced type like A.B.C, we should resolve
            // whatever namespace B is, and add completions for types in it
            if let Some(final_namespace) = typ.resolve_namespace(doc, None) {
                items.extend(completions_in_namespace(doc, final_namespace));
            }
        } else if node.is_named() {
            // Node is A/B in a namespaced type like A.B.C, we should resolve
            // namespaces *up until* this node, and add completions for types
            let parent = node.parent().unwrap();
            let index = parent
                .children_by_field_name("namespace", &mut parent.walk())
                .position(|child| child == node)
                .expect("node is child of its own parent, and is a 'namespace' field");
            if let Some(final_namespace) = typ.resolve_namespace(doc, Some(index)) {
                items.extend(completions_in_namespace(doc, final_namespace));
            }
        }
    }

    items
}

fn completions_in_namespace(doc: &Document, namespace: Node) -> Vec<(CompletionItemKind, String)> {
    let mut items = Vec::new();
    let mut cursor = namespace.walk();

    for child in namespace.children(&mut cursor) {
        let is_namespace = match child.kind() {
            "namespace_declaration" => true,
            "type_declaration" => false,
            _ => continue,
        };
        if let Some(name) = child.child_by_field_name("name") {
            let text = doc.node_text(name);
            let kind = if is_namespace {
                CompletionItemKind::MODULE
            } else {
                CompletionItemKind::VARIABLE
            };
            items.push((kind, text));
        }
    }

    items
}
