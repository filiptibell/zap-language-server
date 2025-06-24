use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{
        find_ancestor, find_child, find_descendant, ts_range_contains_lsp_position,
    },
};
use zap_language::docs::get_primitive_names;

use crate::{structs::DeclaredType, utils::is_type};

pub fn completion(doc: &Document, pos: Position, node: Node) -> Vec<(CompletionItemKind, String)> {
    // If our current node is a top-level "source file" or "namespace_declaration"
    // we can probably drill down to something a bit more specific & useful
    let node = if matches!(node.kind(), "source_file" | "namespace_declaration") {
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

    if is_type(node) {
        items.extend(
            ["struct", "enum", "set", "map"]
                .iter()
                .map(|word| (CompletionItemKind::KEYWORD, (*word).to_string())),
        );

        items.extend(
            get_primitive_names().map(|prim| (CompletionItemKind::CLASS, prim.to_string())),
        );

        let Some(nearest_namespace) = find_ancestor(node, |ancestor| {
            matches!(ancestor.kind(), "source_file" | "namespace_declaration")
        }) else {
            return items;
        };

        items.extend(
            DeclaredType::find_all_in(nearest_namespace)
                .into_iter()
                .filter(|decl| decl.is_in_namespace(nearest_namespace))
                .map(|decl| (CompletionItemKind::VARIABLE, decl.identifier_text(doc))),
        );
    }

    items
}
