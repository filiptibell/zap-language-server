use tree_sitter::Node;

#[must_use]
pub fn is_known_node(node: Node) -> bool {
    is_comment_node(node)
        || is_declaration_node(node)
        || is_type_node(node)
        || is_range_node(node)
        || is_array_node(node)
        || is_ident_node(node)
}

#[must_use]
pub fn is_comment_node(node: Node) -> bool {
    matches!(node.kind(), "comment" | "doc_comment")
}

#[must_use]
pub fn is_declaration_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "option_declaration" | "type_declaration" | "event_declaration" | "function_declaration"
    )
}

#[must_use]
pub fn is_type_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "type"
            | "primitive_type"
            | "optional_type"
            | "struct_type"
            | "enum_type"
            | "map_type"
            | "set_type"
    )
}

#[must_use]
pub fn is_range_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "range" | "range_empty" | "range_exact" | "range_inexact"
    )
}

#[must_use]
pub fn is_array_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "array" | "array_empty" | "array_exact" | "array_inexact"
    )
}

#[must_use]
pub fn is_ident_node(node: Node) -> bool {
    matches!(node.kind(), "identifier")
}
