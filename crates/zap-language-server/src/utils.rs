use async_language_server::{tree_sitter::Node, tree_sitter_utils::find_child};

/**
    Checks if the given node is a type, primitive or reference.
*/
pub fn is_type(node: Node) -> bool {
    is_type_primitive(node) || is_type_reference(node)
}

/**
    Checks if the given node is a primitive type.
*/
pub fn is_type_primitive(node: Node) -> bool {
    matches!(node.kind(), "primitive_type")
}

/**
    Checks if the given identifier node is a type reference.

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
        // These all have an inner "type" field
        "namespaced_type" | "optional_type" | "property" | "set_type" => p
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

        // The encompassing "type" node has at least a single inner
        // child, unnamed, which is the actual type contents of it
        "type" => p.child(0).is_some_and(is_ident_and_matches_this_node),

        // Nothing else can be a type reference according to grammar
        _ => false,
    }
}
