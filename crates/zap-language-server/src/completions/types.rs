use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::ts_range_contains_lsp_position,
};
use zap_language::docs::get_primitive_names;

const TYPE_KEYWORDS: [&str; 4] = ["struct", "enum", "set", "map"];

pub fn completion(
    doc: &Document,
    pos: &Position,
    node: &Node,
    parent: Option<&Node>,
) -> Vec<(CompletionItemKind, String)> {
    let pos = pos.clone();

    let mut parent = parent.cloned();
    let mut node = node.clone();

    // If our current node is the top-level "source file" we can
    // probably drill down to something a bit more specific & useful
    if node.kind() == "source_file" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_declaration" {
                if ts_range_contains_lsp_position(child.range(), pos) {
                    parent = Some(node);
                    node = child;
                    break;
                }
            }
        }
    }

    let mut items = Vec::new();
    let Some(parent) = parent else { return items };

    // We are currently typing inside of a type declaration, so if
    // we are at Y in some "type X = Y" we can complete type names
    let in_type_decl = parent.kind() == "source_file" && node.kind() == "type_declaration";

    // We might be at K or V in either of "set { V }" or "map { [K]: V }"
    // and will just need to make sure, then we can complete type names
    let in_set_or_map = matches!(node.kind(), "set_type" | "map_type");

    // We might be in some kind of property field (or data / args / rets),
    // and will just need to make sure, then we can complete type names
    let in_property = matches!(
        node.kind(),
        "property" | "event_data_field" | "function_args_field" | "function_rets_field"
    );

    if in_type_decl || in_set_or_map || in_property {
        let mut ident = node.child_by_field_name("type");

        if let Some(ident_range) = ident.as_ref().map(|i| i.range()) {
            // Properties have a "type" field, which must be the one we're completing,
            // otherwise we'd be completing identifiers when inside the property name
            if !ts_range_contains_lsp_position(ident_range, pos) {
                ident = None;
            }
        } else {
            // Not a property, meaning we are in data / args / rets, which only
            // have a single identifier, and that is guaranteed to be the type
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "identifier" {
                    if ts_range_contains_lsp_position(child.range(), pos) {
                        ident = Some(child);
                        break;
                    }
                }
            }
        }

        if ident.is_some() {
            items.extend(
                TYPE_KEYWORDS
                    .iter()
                    .map(|word| (CompletionItemKind::KEYWORD, word.to_string())),
            );

            items.extend(
                get_primitive_names().map(|prim| (CompletionItemKind::CLASS, prim.to_string())),
            );

            let root = doc.node_at_root().unwrap();
            let mut root_cursor = root.walk();

            let mut declared_types = Vec::new();
            for top_level in root.children(&mut root_cursor) {
                if top_level.kind() == "type_declaration" {
                    let mut top_level_cursor = top_level.walk();
                    for child in top_level.children(&mut top_level_cursor) {
                        if child.kind() == "identifier" {
                            let type_name = doc.text().slice(child.byte_range());
                            declared_types.push(type_name.to_string());
                            break;
                        }
                    }
                }
            }

            items.extend(
                declared_types
                    .into_iter()
                    .map(|name| (CompletionItemKind::VARIABLE, name)),
            );
        }
    }

    items
}
