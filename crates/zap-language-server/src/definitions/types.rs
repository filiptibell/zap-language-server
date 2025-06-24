use async_language_server::{
    lsp_types::{GotoDefinitionResponse, Location, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::structs::ReferencedType;

pub fn definition(doc: &Document, _pos: Position, node: Node) -> Option<GotoDefinitionResponse> {
    // May be a referenced type that needs to be resolved,
    // if we are selecting a qualified / namespaced type we
    // should also make sure to resolve the *full* reference
    let node = match node.parent() {
        Some(p) if p.kind() == "namespaced_type" => p,
        _ => node,
    };

    let typ = ReferencedType::from_node(doc, node)?;
    let decl = typ.resolve_declaration()?;

    Some(GotoDefinitionResponse::Scalar(Location {
        uri: doc.url().clone(),
        range: ts_range_to_lsp_range(decl.identifier_range()),
    }))
}
