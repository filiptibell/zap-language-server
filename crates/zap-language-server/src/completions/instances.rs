use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::find_child,
};

use zap_language::docs::get_instance_class_names;

pub fn completion(doc: &Document, _pos: Position, node: Node) -> Vec<(CompletionItemKind, String)> {
    // We may have gotten the specific "identifier"
    // node inside of a range, travel up from there
    let mut node = node;
    if matches!(node.kind(), "identifier") {
        let Some(parent) = node.parent() else {
            return vec![];
        };
        node = parent;
    }

    // We should now be inside some kind of range node to
    // autocomplete instance class names, or more specifically,
    // an empty range, or one with a partial identifier / class name
    if !matches!(node.kind(), "range_empty" | "range_ident") {
        return vec![];
    }

    // The parent node must then be the top-level "range" node,
    // which itself is then contained in a top-level "type" node
    let parent = node.parent().unwrap();
    let grandparent = parent.parent().unwrap();

    // The top-level "type" node must then also be an Instance
    // primitive to provide completions for instance class names
    if find_child(grandparent, |c| {
        let is_primitive = c.kind() == "primitive_type";
        let is_instance = doc
            .text()
            .byte_slice(c.byte_range())
            .as_str()
            .is_some_and(|s| s == "Instance");
        is_primitive && is_instance
    })
    .is_none()
    {
        return vec![];
    }

    // Return all possible class names and
    // let the editor filter when typing
    get_instance_class_names()
        .map(|name| (CompletionItemKind::VALUE, name.to_string()))
        .collect()
}
