use async_language_server::tree_sitter::Node;

pub fn is_type_reference(node: Node) -> bool {
    node.parent().is_some_and(|p| {
        if matches!(p.kind(), "property" | "set_type" | "map_type") {
            return p
                .child_by_field_name("type")
                .or_else(|| p.child_by_field_name("key_type"))
                .or_else(|| p.child_by_field_name("value_type"))
                .is_some_and(|type_field| type_field == node && type_field.kind() == "identifier");
        } else if matches!(
            p.kind(),
            "event_data_field" | "function_args_field" | "function_rets_field"
        ) {
            let mut cursor = p.walk();
            for child in p.children(&mut cursor) {
                if child == node && child.kind() == "identifier" {
                    return true;
                }
            }
        }
        false
    })
}
