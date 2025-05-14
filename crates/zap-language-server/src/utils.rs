#![allow(clippy::needless_pass_by_value)]

use std::collections::HashMap;

use async_language_server::{server::Document, tree_sitter::Node, tree_sitter_utils::find_child};

/**
    Gathers all of the declared types in the given document, and returns
    them all as a `HashMap` of type names to their corresponding nodes.
*/
pub fn gather_declared_types(doc: &Document) -> HashMap<String, Node> {
    let Some(root) = doc.node_at_root() else {
        return HashMap::new();
    };

    let mut cursor = root.walk();
    let mut types = HashMap::new();

    for top_level in root.children(&mut cursor) {
        if top_level.kind() == "type_declaration" {
            let mut top_level_cursor = top_level.walk();
            for child in top_level.children(&mut top_level_cursor) {
                if child.kind() == "identifier" {
                    let text = doc.text().byte_slice(child.byte_range());
                    if let Some(text) = text.as_str() {
                        types.insert(text.to_string(), top_level);
                    }
                    break;
                }
            }
        }
    }

    types
}

/**
    Finds a specific declared type by its name in the given document,
    and returns the corresponding node for the full type declaration.
*/
pub fn find_declared_type(doc: &Document, type_name: impl ToString) -> Option<Node> {
    let root = doc.node_at_root()?;
    let name = type_name.to_string();

    let mut cursor = root.walk();
    for top_level in root.children(&mut cursor) {
        if top_level.kind() == "type_declaration" {
            let mut top_level_cursor = top_level.walk();
            for child in top_level.children(&mut top_level_cursor) {
                if child.kind() == "identifier" {
                    let text = doc.text().byte_slice(child.byte_range());
                    if text.as_str().is_some_and(|t| t == name.as_str()) {
                        return Some(top_level);
                    }
                }
            }
        }
    }

    None
}

/**
    Checks if the given node is a primitive type.
*/
pub fn is_type_primitive(node: Node) -> bool {
    matches!(node.kind(), "primitive_type")
}

/**
    Checks if the given node is a type, primitive or reference.
*/
pub fn is_type(node: Node) -> bool {
    is_type_primitive(node) || is_type_reference(node)
}

/**
    Checks if the given node is a type reference.

    Note that this does not check if the node is a **valid** type
    reference, only that it is *trying* to reference some type.
*/
pub fn is_type_reference(node: Node) -> bool {
    // Type references are always classified as "identifier"
    let is_ident_and_matches_this_node =
        |child: Node| child == node && child.kind() == "identifier";

    // All type references have some kind of parent node
    let Some(p) = node.parent() else {
        return false;
    };

    match p.kind() {
        // Properties and sets have an inner "type" field
        "property" | "set_type" => p
            .child_by_field_name("type")
            .is_some_and(is_ident_and_matches_this_node),

        // Maps have two inner fields "key_type" and "value_type"
        "map_type" => {
            p.child_by_field_name("key_type")
                .is_some_and(is_ident_and_matches_this_node)
                || p.child_by_field_name("value_type")
                    .is_some_and(is_ident_and_matches_this_node)
        }

        // Type declarations have an inner "value" field
        "type_declaration" => p
            .child_by_field_name("value")
            .is_some_and(is_ident_and_matches_this_node),

        // The data / args / rets fields have a single child that
        // is not a keyword and is either a primitive or identifier
        "event_data_field" | "function_args_field" | "function_rets_field" => {
            find_child(p, is_ident_and_matches_this_node).is_some()
        }

        // Nothing else can be a type reference according to grammar
        _ => false,
    }
}
