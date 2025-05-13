use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_contains_lsp_position,
};

use crate::docs::get_option_names;

pub fn completion(
    doc: &Document,
    pos: &Position,
    node: &Node,
    parent: Option<&Node>,
) -> Vec<(CompletionItemKind, String)> {
    let pos = pos.clone();

    let mut node = node.clone();
    let Some(mut parent) = parent.cloned() else {
        return Vec::new();
    };

    if node.kind() == "option_declaration" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                if ts_range_contains_lsp_position(child.range(), pos) {
                    parent = node;
                    node = child;
                    break;
                }
            }
        }
    }

    let mut items = Vec::new();

    if parent.kind() == "option_declaration" && node.kind() == "identifier" {
        let ident = doc.text().byte_slice(node.byte_range());
        if let Some(ident) = ident.as_str() {
            items.extend(
                get_option_names()
                    .filter(|opt| opt.contains(ident))
                    .map(|opt| (CompletionItemKind::ENUM_MEMBER, opt.to_string())),
            );
        }
    }

    items
}
