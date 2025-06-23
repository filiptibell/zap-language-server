use async_language_server::{
    lsp_types::{Hover, HoverContents, MarkedString, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{ts_range_contains_lsp_position, ts_range_to_lsp_range},
};

use zap_language::docs::find_keyword;

pub fn hover(doc: &Document, pos: Position, node: Node) -> Option<Hover> {
    let mut node = node;

    // When hovering over type declarations of some kind, the actual
    // node target will be the entire declaration, we need to descend
    // to the first child which is guaranteed to be a keyword there
    if matches!(
        node.kind(),
        "type_declaration"
            | "event_declaration"
            | "function_declaration"
            | "namespace_declaration"
            | "map_type"
            | "set_type"
            | "struct_type"
            | "enum_type"
    ) {
        node = node.child(0)?;
        if !ts_range_contains_lsp_position(node.range(), pos) {
            return None; // Probably hovering over '{}' or '=', not the keyword
        }
    }

    let text = doc.text().byte_slice(node.byte_range());
    if let Some((head, desc)) = find_keyword([text]) {
        return Some(Hover {
            range: Some(ts_range_to_lsp_range(node.range())),
            contents: HoverContents::Scalar(MarkedString::String(format!("# {head}\n\n{desc}\n"))),
        });
    }

    None
}
