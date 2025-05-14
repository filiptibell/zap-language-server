use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_contains_lsp_position,
};

const KEYWORDS: [&str; 4] = ["type", "opt", "event", "funct"];

pub fn completion(
    _doc: &Document,
    pos: &Position,
    node: &Node,
    parent: Option<&Node>,
) -> Vec<(CompletionItemKind, String)> {
    let pos = *pos;

    let mut parent = parent.copied();
    let mut node = *node;

    // If our current node is the top-level "source file" we can
    // probably drill down to something a bit more specific & useful
    if node.kind() == "source_file" {
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
