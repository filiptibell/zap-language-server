use async_language_server::{
    lsp_types::{Location, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::structs::{DeclaredNamespace, ReferencedNamespace};

pub fn references(doc: &Document, _pos: Position, node: Node) -> Option<Vec<Location>> {
    // 1. Transform the identifier node we are possibly on, into the
    //    full node for the declaration / reference, when possible
    let node = match node.parent() {
        Some(p) if matches!(p.kind(), "namespace_declaration" | "namespaced_type") => p,
        _ => node,
    };

    // 2. Resolve the namespace declaration
    let declaration = match DeclaredNamespace::from_node(node) {
        Some(decl) => decl,
        None => match ReferencedNamespace::from_node(node) {
            Some(ns) => ns.resolve_declaration(doc)?,
            None => return None,
        },
    };

    // 3. We have a definite declaration, so we can resolve references
    let url = doc.url().clone();
    let locations = declaration
        .resolve_references(doc)
        .into_iter()
        .map(|ns| Location {
            uri: url.clone(),
            range: ts_range_to_lsp_range(ns.identifier_range()),
        })
        .collect();

    Some(locations)
}
