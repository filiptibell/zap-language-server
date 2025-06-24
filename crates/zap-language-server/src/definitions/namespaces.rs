use async_language_server::{
    lsp_types::{GotoDefinitionResponse, Location, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::structs::ReferencedNamespace;

pub fn definition(doc: &Document, _pos: Position, node: Node) -> Option<GotoDefinitionResponse> {
    // For namespace definitions, we want to resolve namespace identifiers
    // within namespaced types to their declarations
    let ns = ReferencedNamespace::from_node(node)?;
    let decl = ns.resolve_declaration(doc)?;

    Some(GotoDefinitionResponse::Scalar(Location {
        uri: doc.url().clone(),
        range: ts_range_to_lsp_range(decl.identifier_range()),
    }))
}
