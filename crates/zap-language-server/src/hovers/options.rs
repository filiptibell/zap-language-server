use async_language_server::{
    lsp_types::{Hover, HoverContents, MarkedString, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use zap_language::docs::find_option;

pub fn hover(doc: &Document, _pos: Position, node: Node, _parent: Option<Node>) -> Option<Hover> {
    let text = doc.text().byte_slice(node.byte_range());

    if let Some((name, _, desc)) = find_option([text]) {
        return Some(Hover {
            range: Some(ts_range_to_lsp_range(node.range())),
            contents: HoverContents::Scalar(MarkedString::String(format!("# {name}\n\n{desc}\n"))),
        });
    }

    None
}
