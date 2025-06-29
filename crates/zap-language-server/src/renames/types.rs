use std::collections::HashMap;

use async_language_server::{
    lsp_types::{Position, PrepareRenameResponse, TextEdit, WorkspaceEdit},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{ts_range_contains_lsp_position, ts_range_to_lsp_range},
};

use crate::structs::{DeclaredType, ReferencedType};

pub fn prepare(_doc: &Document, pos: Position, node: Node) -> Option<PrepareRenameResponse> {
    // 1. Check if we are renaming the identifier part of a declaration
    if let Some(decl) = node.parent().and_then(DeclaredType::from_node) {
        if ts_range_contains_lsp_position(decl.identifier_range(), pos) {
            return Some(PrepareRenameResponse::Range(ts_range_to_lsp_range(
                decl.identifier_range(),
            )));
        }
    }

    // 2. Check if we are renaming the identifier part of a reference
    let node = if node.kind() == "namespaced_type" {
        node.child_by_field_name("type")?
    } else {
        node
    };
    if let Some(typ) = ReferencedType::from_node(node) {
        if ts_range_contains_lsp_position(typ.identifier_range(), pos) {
            return Some(PrepareRenameResponse::Range(ts_range_to_lsp_range(
                typ.identifier_range(),
            )));
        }
    }

    None
}

pub fn rename(doc: &Document, pos: Position, node: Node, new_name: &str) -> Option<WorkspaceEdit> {
    // 1. Transform the identifier node we are possibly on, into the
    //    full node for the declaration / reference, when possible
    let node = match node.parent() {
        Some(p) if matches!(p.kind(), "type_declaration" | "namespaced_type") => p,
        _ => node,
    };

    // 2. Find the type declaration and all type references to it,
    //    also making sure that our position is over an identifier
    let declaration = match DeclaredType::from_node(node) {
        Some(decl) => {
            if ts_range_contains_lsp_position(decl.identifier_range(), pos) {
                decl
            } else {
                return None;
            }
        }
        None => match ReferencedType::from_node(node) {
            Some(typ) => {
                if ts_range_contains_lsp_position(typ.identifier_range(), pos) {
                    typ.resolve_declaration(doc)?
                } else {
                    return None;
                }
            }
            None => return None,
        },
    };

    // 3. Edit the type declaration
    let mut edits = vec![TextEdit {
        range: ts_range_to_lsp_range(declaration.identifier_range()),
        new_text: new_name.to_string(),
    }];

    // 4. Edit any references to the type
    for type_reference in declaration.resolve_references(doc) {
        edits.push(TextEdit {
            range: ts_range_to_lsp_range(type_reference.identifier_range()),
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
