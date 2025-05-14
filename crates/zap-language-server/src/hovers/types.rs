use async_language_server::{
    lsp_types::{Hover, HoverContents, MarkedString, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};
use zap_language::docs::find_primitive;

use crate::utils::{find_declared_type, is_type, is_type_reference};

pub fn hover(doc: &Document, _pos: Position, node: Node) -> Option<Hover> {
    if !is_type(node) {
        return None;
    }

    let text = doc.text().byte_slice(node.byte_range());

    if let Some((_, header, desc)) = find_primitive([text]) {
        return Some(Hover {
            range: Some(ts_range_to_lsp_range(node.range())),
            contents: HoverContents::Scalar(MarkedString::String(format!(
                "# {header}\n\n{desc}\n"
            ))),
        });
    }

    if is_type_reference(node) {
        let type_decl = find_declared_type(doc, text)?;
        let type_contents = doc.text().byte_slice(type_decl.byte_range());

        return Some(Hover {
            range: Some(ts_range_to_lsp_range(node.range())),
            contents: HoverContents::Scalar(MarkedString::String(format!(
                "```zap\n{type_contents}\n```"
            ))),
        });
    }

    None
}
