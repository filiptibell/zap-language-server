use async_language_server::{
    lsp_types::{Hover, HoverContents, MarkedString, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{lsp_position_to_ts_point, ts_range_to_lsp_range},
};

use zap_language::{docs::find_property, tree_sitter_utils::is_punctuation_str};

pub fn hover(doc: &Document, pos: Position, node: Node) -> Option<Hover> {
    let parent = node.parent()?;

    if let Some((prop_name, head, desc)) = find_property([parent.kind(), node.kind()]) {
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

        // Not hovering the property name, and not hovering a special
        // enum-like value, meaning probably hovering over a user type
        if exact_text != prop_name && !node.kind().ends_with("_value") {
            return None;
        }

        return Some(Hover {
            range: Some(ts_range_to_lsp_range(exact_node.range())),
            contents: HoverContents::Scalar(MarkedString::String(format!("# {head}\n\n{desc}\n"))),
        });
    }

    None
}
