use async_language_server::{
    lsp_types::{CompletionItem, CompletionItemKind, CompletionResponse, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_contains_lsp_position,
};

use crate::docs::get_option_names;

pub fn completion(
    doc: &Document,
    pos: &Position,
    node: &Node,
    parent: &Node,
) -> Option<CompletionResponse> {
    let pos = pos.clone();
    let mut node = node.clone();
    let mut parent = parent.clone();

    if node.kind() == "option_declaration" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                if ts_range_contains_lsp_position(child.range(), pos) {
                    parent = node;
                    node = child;
                }
                break;
            }
        }
    }

    if parent.kind() == "option_declaration" && node.kind() == "identifier" {
        let ident = doc.text().byte_slice(node.byte_range());
        if let Some(ident) = ident.as_str() {
            let mut completions = get_option_names()
                .filter(|opt| opt.contains(ident))
                .map(ToString::to_string)
                .collect::<Vec<_>>();

            completions.sort_unstable();
            completions.dedup();

            return Some(CompletionResponse::Array(
                completions
                    .into_iter()
                    .map(|opt| CompletionItem {
                        kind: Some(CompletionItemKind::ENUM_MEMBER),
                        label: opt,
                        ..Default::default()
                    })
                    .collect(),
            ));
        }
    }

    None
}
