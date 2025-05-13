use async_language_server::{
    lsp_types::{GotoDefinitionResponse, Location, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::utils::is_type_reference;

pub fn definition(
    doc: &Document,
    _pos: &Position,
    node: &Node,
    _parent: Option<&Node>,
) -> Option<GotoDefinitionResponse> {
    let node_text = doc.text().slice(node.byte_range());
    if node_text.as_str().is_none_or(|t| t.is_empty()) || !is_type_reference(node) {
        return None;
    }

    let root_node = doc.node_at_root().unwrap();
    let mut root_cursor = root_node.walk();

    for top_level in root_node.children(&mut root_cursor) {
        if top_level.kind() == "type_declaration" {
            let mut top_level_cursor = top_level.walk();
            for child in top_level.children(&mut top_level_cursor) {
                if child.kind() == "identifier" {
                    let type_name = doc.text().slice(child.byte_range());
                    if type_name.as_str().is_some_and(|n| n == node_text) {
                        return Some(GotoDefinitionResponse::Scalar(Location {
                            uri: doc.url().clone(),
                            range: ts_range_to_lsp_range(child.range()),
                        }));
                    }
                }
            }
        }
    }

    None
}
