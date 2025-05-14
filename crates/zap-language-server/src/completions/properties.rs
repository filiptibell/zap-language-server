use async_language_server::{
    lsp_types::{CompletionItemKind, Position},
    server::Document,
    tree_sitter::Node,
};
use zap_language::docs::find_variants;

pub fn completion(doc: &Document, _pos: Position, node: Node) -> Vec<(CompletionItemKind, String)> {
    let Some(parent) = node.parent() else {
        return Vec::new();
    };

    let mut items = Vec::new();

    let is_event = parent.kind().starts_with("event_");
    let is_funct = parent.kind().starts_with("function_");
    let is_field = parent.kind().ends_with("_field");
    if (is_event || is_funct) && is_field {
        let field_name = doc.text().byte_slice(parent.byte_range());
        if let Some((false, variants)) = find_variants([field_name]) {
            items.extend(
                variants
                    .iter()
                    .map(|word| (CompletionItemKind::ENUM_MEMBER, (*word).to_string())),
            );
        }
    }

    items
}
