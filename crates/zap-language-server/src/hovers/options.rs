use async_language_server::{
    lsp_types::{Hover, HoverContents, MarkedString, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{lsp_position_to_ts_point, ts_range_to_lsp_range},
};

use zap_language::{docs::find_option, tree_sitter_utils::is_punctuation_str};

pub fn hover(doc: &Document, pos: Position, node: Node) -> Option<Hover> {
    let text = doc.text().byte_slice(node.byte_range());

    if let Some((name, _, desc)) = find_option([text]) {
        let point = lsp_position_to_ts_point(pos);

        let exact_node = node
            .descendant_for_point_range(point, point)
            .unwrap_or(node);
        let exact_text = doc.text().byte_slice(exact_node.byte_range());
        let exact_text = exact_text.as_str()?;

        // Hovering over punctuation
        if is_punctuation_str(exact_text) {
            return None;
        }

        return Some(Hover {
            range: Some(ts_range_to_lsp_range(node.range())),
            contents: HoverContents::Scalar(MarkedString::String(format!("# {name}\n\n{desc}\n"))),
        });
    }

    None
}
