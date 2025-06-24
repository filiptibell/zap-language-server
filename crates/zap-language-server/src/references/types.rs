use async_language_server::{
    lsp_types::{Location, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::structs::{DeclaredType, ReferencedType};

pub fn references(doc: &Document, _pos: Position, node: Node) -> Option<Vec<Location>> {
    // 1. Transform the identifier node we are possibly on, into the
    //    full node for the declaration / reference, when possible
    let node = match node.parent() {
        Some(p) if matches!(p.kind(), "type_declaration" | "namespaced_type") => p,
        _ => node,
    };

    // 2. Resolve the type declaration
    let declaration = match DeclaredType::from_node(node) {
        Some(decl) => decl,
        None => match ReferencedType::from_node(node) {
            Some(typ) => typ.resolve_declaration(doc)?,
            None => return None,
        },
    };

    // 3. We have a definite declaration, so we can resolve references
    let url = doc.url().clone();
    let locations = declaration
        .resolve_references(doc)
        .into_iter()
        .map(|typ| Location {
            uri: url.clone(),
            range: ts_range_to_lsp_range(typ.identifier_range()),
        })
        .collect();

    Some(locations)
}
