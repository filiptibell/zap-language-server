use async_language_server::{
    lsp_types::{Hover, HoverContents, MarkedString, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::docs::{find_enum_docs, find_option_docs};

pub fn hover(doc: &Document, _pos: &Position, node: &Node, parent: &Node) -> Option<Hover> {
    if let Some((head, desc)) = find_enum_docs([parent.kind(), node.kind()]).or_else(|| {
        if parent.kind() == "option_declaration" && node.kind() == "identifier" {
            let ident = doc.text().byte_slice(node.byte_range());
            find_option_docs([ident])
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
