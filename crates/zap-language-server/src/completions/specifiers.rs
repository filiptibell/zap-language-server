use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{
        find_ancestor, find_child, find_descendant, ts_range_contains_lsp_position,
    },
};

use zap_language::docs::get_instance_class_names;

use crate::utils::{is_namespace, is_type_primitive};

pub fn completion(doc: &Document, pos: Position, node: Node) -> Vec<(CompletionItemKind, String)> {
    // If our current node is a top-level, we can probably
    // find something that is a bit more specific & useful
    let node = if is_namespace(node) {
        find_child(node, |c| {
            ts_range_contains_lsp_position(c.range(), pos) && c.kind() == "type_declaration"
        })
        .unwrap_or(node)
    } else {
        node
    };

    // Try to find an even more specific node, if possible
    let node = find_descendant(node, |d| {
        ts_range_contains_lsp_position(d.range(), pos) && is_type_primitive(d)
    })
    .unwrap_or(node);

    // If we are inside a descendant node of a primitive
    // type, we should traverse up to the main type node
    let node = find_ancestor(node, |a| is_type_primitive(a)).unwrap_or(node);

    // The primitive type node must then also be either a "string"
    // or "Instance" primitive to provide completions for specifiers
    let Some((kind_node, kind)) = find_specifier_kind(doc, node) else {
        return vec![];
    };

    // Finally, make sure we are actually fetching completions for the
    // specifier part, and not for inside "string" or "Instance" part
    if ts_range_contains_lsp_position(kind_node.range(), pos) {
        return vec![];
    }

    match kind {
        SpecifierKind::String => {
            // String specifiers can only be utf8 or binary
            vec![
                (CompletionItemKind::VALUE, String::from("utf8")),
                (CompletionItemKind::VALUE, String::from("binary")),
            ]
        }
        SpecifierKind::Instance => {
            // Return all possible class names and
            // let the editor filter when typing
            get_instance_class_names()
                .map(|name| (CompletionItemKind::VALUE, name.to_string()))
                .collect()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SpecifierKind {
    String,
    Instance,
}

fn find_specifier_kind<'a>(doc: &Document, node: Node<'a>) -> Option<(Node<'a>, SpecifierKind)> {
    let child_node = node.child(0)?;
    let child_text = doc.text().byte_slice(child_node.byte_range()).as_str()?;

    if child_text == "string" {
        Some((child_node, SpecifierKind::String))
    } else if child_text == "Instance" {
        Some((child_node, SpecifierKind::Instance))
    } else {
        None
    }
}
