use async_language_server::{
    lsp_types::{Location, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::utils::{gather_type_references, is_declared_type, is_type_reference};

pub fn references(doc: &Document, _pos: Position, node: Node) -> Option<Vec<Location>> {
    if !is_declared_type(node) && !is_type_reference(node) {
        return None;
    }

    let type_name = doc.text().byte_slice(node.byte_range()).to_string();
    tracing::info!("Finding references for type '{type_name}'");

    let type_references = gather_type_references(doc.node_at_root()?)
        .into_iter()
        .filter(|type_reference| {
            let type_str = doc.text().byte_slice(type_reference.byte_range());
            type_str
                .as_str()
                .is_some_and(|s| s.trim() == type_name.as_str())
        });

    let url = doc.url().clone();
    let locations = type_references
        .map(|node| Location {
            uri: url.clone(),
            range: ts_range_to_lsp_range(node.range()),
        })
        .collect();

    Some(locations)
}
