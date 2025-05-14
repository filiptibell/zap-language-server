use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{find_child, ts_range_contains_lsp_position},
};

const KEYWORDS: [&str; 4] = ["type", "opt", "event", "funct"];

pub fn completion(_doc: &Document, pos: Position, node: Node) -> Vec<(CompletionItemKind, String)> {
    // If our current node is the top-level "source file" we can
    // probably drill down to something a bit more specific & useful
    let node = if node.kind() == "source_file" {
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

    if parent.kind() == "source_file" && node.kind() == "identifier" {
        // We are currently typing some kind of identifier
        // at the top level of the file, without anything
        // else, so assume its a start of a new declaration
        items.extend(
            KEYWORDS
                .iter()
                .map(|word| (CompletionItemKind::KEYWORD, (*word).to_string())),
        );
    }

    items
}
