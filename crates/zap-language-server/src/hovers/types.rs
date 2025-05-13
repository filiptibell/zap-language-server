use async_language_server::{
    lsp_types::{Hover, HoverContents, MarkedString, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{ts_range_contains_lsp_position, ts_range_to_lsp_range},
};

pub fn hover(doc: &Document, pos: &Position, node: &Node, parent: Option<&Node>) -> Option<Hover> {
    if node.kind() != "identifier" {
        return None;
    }

    let parent = parent?;
    let pos = pos.clone();

    // We might be at K or V in either of "set { V }" or "map { [K]: V }",
    // and will just need to make sure, then we can show the type on hover
    let in_set_or_map = matches!(parent.kind(), "set_type" | "map_type");

    // We might be in some kind of property field (or data / args / rets),
    // and will just need to make sure, then we can show the type on hover
    let in_property = matches!(
        parent.kind(),
        "property" | "event_data_field" | "function_args_field" | "function_rets_field"
    );

    if in_set_or_map || in_property {
        let mut ident = parent.child_by_field_name("type");

        if let Some(ident_range) = ident.as_ref().map(|i| i.range()) {
            // Properties have a "type" field, which must be the one we're hovering
            if !ts_range_contains_lsp_position(ident_range, pos) {
                ident = None;
            }
        } else {
            // Not a property, meaning we are in data / args / rets, which only
            // have a single identifier, and that is guaranteed to be the type
            let mut cursor = parent.walk();
            for child in parent.children(&mut cursor) {
                if child.kind() == "identifier" {
                    if ts_range_contains_lsp_position(child.range(), pos) {
                        ident = Some(child);
                        break;
                    }
                }
            }
        }

        if ident.is_some_and(|i| i == *node) {
            let node_text = doc.text().byte_slice(node.byte_range());

            let root = doc.node_at_root()?;
            let mut root_cursor = root.walk();

            let mut matching_types = Vec::new();
            for top_level in root.children(&mut root_cursor) {
                if top_level.kind() == "type_declaration" {
                    let mut top_level_cursor = top_level.walk();
                    for child in top_level.children(&mut top_level_cursor) {
                        if child.kind() == "identifier" {
                            let child_text = doc.text().byte_slice(child.byte_range());
                            if child_text == node_text {
                                matching_types.push(child);
                                break;
                            }
                        }
                    }
                }
            }

            // Must have exactly one type for hover to not be ambiguous
            // or pointing to something the user may not want to see
            if matching_types.len() == 1 {
                let type_node_name = matching_types.pop().unwrap();
                let type_node = type_node_name.parent().unwrap();
                let type_text = doc.text().byte_slice(type_node.byte_range());
                return Some(Hover {
                    range: Some(ts_range_to_lsp_range(node.range())),
                    contents: HoverContents::Scalar(MarkedString::String(format!(
                        "```zap\n{type_text}\n```"
                    ))),
                });
            }
        }
    }

    None
}
