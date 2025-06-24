use std::collections::HashMap;

use async_language_server::{
    lsp_types::{Position, PrepareRenameResponse, TextEdit, WorkspaceEdit},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{ts_range_contains_lsp_position, ts_range_to_lsp_range},
};

use crate::structs::{DeclaredNamespace, ReferencedNamespace};

pub fn prepare(_doc: &Document, pos: Position, node: Node) -> Option<PrepareRenameResponse> {
    // 1. Check if we are renaming the identifier part of a namespace declaration
    if let Some(decl) = node.parent().and_then(DeclaredNamespace::from_node) {
        if ts_range_contains_lsp_position(decl.identifier_range(), pos) {
            return Some(PrepareRenameResponse::Range(ts_range_to_lsp_range(
                decl.identifier_range(),
            )));
        }
    }

    // 2. Check if we are renaming a namespace identifier in a namespaced type
    if let Some(ns_ref) = ReferencedNamespace::from_node(node) {
        if ts_range_contains_lsp_position(ns_ref.identifier_range(), pos) {
            return Some(PrepareRenameResponse::Range(ts_range_to_lsp_range(
                ns_ref.identifier_range(),
            )));
        }
    }

    None
}

pub fn rename(doc: &Document, _pos: Position, node: Node, new_name: &str) -> Option<WorkspaceEdit> {
    // 1. Transform the identifier node we are possibly on, into the
    //    full node for the declaration / reference, when possible
    let node = match node.parent() {
        Some(p) if p.kind() == "namespace_declaration" => p,
        _ => node,
    };

    // 2. Find the namespace declaration for the node we're renaming
    let declaration = match DeclaredNamespace::from_node(node) {
        Some(decl) => decl,
        None => match ReferencedNamespace::from_node(node) {
            Some(ns_ref) => ns_ref.resolve_declaration(doc)?,
            None => return None,
        },
    };

    // 3. Edit the namespace declaration
    let mut edits = vec![TextEdit {
        range: ts_range_to_lsp_range(declaration.identifier_range()),
        new_text: new_name.to_string(),
    }];

    // 4. Edit any references to the namespace
    for namespace_reference in declaration.resolve_references(doc) {
        edits.push(TextEdit {
            range: ts_range_to_lsp_range(namespace_reference.identifier_range()),
            new_text: new_name.to_string(),
        });
    }

    // 5. Finally, build the full change set
    let url = doc.url().clone();
    let changes = HashMap::from([(url, edits)]);

    Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    })
}
