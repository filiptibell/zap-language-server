use std::collections::HashMap;

use async_language_server::{
    lsp_types::{Position, PrepareRenameResponse, TextEdit, WorkspaceEdit},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_to_lsp_range,
};

use crate::utils::{
    find_declared_type, gather_type_references, is_declared_type, is_type_reference,
};

pub fn prepare(doc: &Document, _pos: Position, node: Node) -> Option<PrepareRenameResponse> {
    create_edits(doc, node, "temp").map(|_| PrepareRenameResponse::DefaultBehavior {
        default_behavior: true,
    })
}

pub fn rename(doc: &Document, _pos: Position, node: Node, new_name: &str) -> Option<WorkspaceEdit> {
    let edits = create_edits(doc, node, new_name)?;

    let url = doc.url().clone();
    let changes = HashMap::from([(url, edits)]);

    Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    })
}

fn create_edits(doc: &Document, node: Node, new_name: &str) -> Option<Vec<TextEdit>> {
    if !is_declared_type(node) && !is_type_reference(node) {
        return None;
    }

    let type_name = doc.text().byte_slice(node.byte_range()).to_string();
    tracing::info!("Renaming type from '{type_name}' to '{new_name}'");

    // 1. Find the type declaration and all type references to it
    let type_decl_node = find_declared_type(doc, &type_name)?;
    let type_decl_ident = type_decl_node.child_by_field_name("name")?;
    let type_references = gather_type_references(doc.node_at_root().unwrap())
        .into_iter()
        .filter(|type_reference| {
            let type_str = doc.text().byte_slice(type_reference.byte_range());
            type_str
                .as_str()
                .is_some_and(|s| s.trim() == type_name.as_str())
        });

    // 2. Edit the type declaration
    let mut edits = vec![TextEdit {
        range: ts_range_to_lsp_range(type_decl_ident.range()),
        new_text: new_name.to_string(),
    }];

    // 3. Edit any references to the type
    for type_reference in type_references {
        edits.push(TextEdit {
            range: ts_range_to_lsp_range(type_reference.range()),
            new_text: new_name.to_string(),
        });
    }

    Some(edits)
}
