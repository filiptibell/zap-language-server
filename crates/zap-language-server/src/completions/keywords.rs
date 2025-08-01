use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{find_child, ts_range_contains_lsp_position},
};

use crate::utils::is_namespace;

const KEYWORDS: [&str; 5] = ["type", "opt", "event", "funct", "namespace"];

pub fn completion(_doc: &Document, pos: Position, node: Node) -> Vec<(CompletionItemKind, String)> {
    // If our current node is a top-level, we can probably
    // find something that is a bit more specific & useful
    let node = if is_namespace(node) {
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

    if node.kind() == "identifier" && is_namespace(parent) {
        // We are currently typing some kind of identifier inside either
        // a namespace or the top level of the file, without anything
        // else, so assume its a start of a new declaration
        items.extend(
            KEYWORDS
                .iter()
                .map(|word| (CompletionItemKind::KEYWORD, (*word).to_string())),
        );
    }

    items
}
