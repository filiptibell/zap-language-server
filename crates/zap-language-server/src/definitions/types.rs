use async_language_server::{
    lsp_types::{GotoDefinitionResponse, Location, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::utils::{find_declared_type, is_type_reference};

pub fn definition(doc: &Document, _pos: Position, node: Node) -> Option<GotoDefinitionResponse> {
    if is_type_reference(node) {
        let type_name = doc.text().byte_slice(node.byte_range());
        let type_decl = find_declared_type(doc, type_name)?;

        return Some(GotoDefinitionResponse::Scalar(Location {
            uri: doc.url().clone(),
            range: ts_range_to_lsp_range(type_decl.range()),
        }));
    }

    None
}
