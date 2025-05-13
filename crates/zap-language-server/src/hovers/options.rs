use async_language_server::{
    lsp_types::{Hover, HoverContents, MarkedString, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::docs::{find_option, find_property};

pub fn hover(doc: &Document, _pos: &Position, node: &Node, parent: Option<&Node>) -> Option<Hover> {
    let parent = parent?.clone();
    let node = node.clone();

    if let Some((head, desc)) = find_property([parent.kind(), node.kind()]).or_else(|| {
        if parent.kind() == "option_declaration" && node.kind() == "identifier" {
            let ident = doc.text().byte_slice(node.byte_range());
            find_option([ident]).map(|(name, _, desc)| (name, desc))
        } else {
            None
        }
    }) {
        return Some(Hover {
            range: Some(ts_range_to_lsp_range(node.range())),
            contents: HoverContents::Scalar(MarkedString::String(format!("# {head}\n\n{desc}\n"))),
        });
    }

    None
}
