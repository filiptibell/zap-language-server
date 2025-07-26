use async_language_server::{
    lsp_types::{Hover, HoverContents, MarkedString, Position},
    server::Document,
    tree_sitter::Node,
    tree_sitter_utils::{find_ancestor, ts_range_to_lsp_range},
};
use zap_language::docs::find_primitive;

use crate::{structs::ReferencedType, utils::is_type_primitive};

pub fn hover(doc: &Document, _pos: Position, node: Node) -> Option<Hover> {
    // If we are inside a descendant node of a primitive
    // type, we should traverse up to the main type node
    let node = find_ancestor(node, |a| is_type_primitive(a)).unwrap_or(node);

    if matches!(node.kind(), "primitive_type") {
        // Primitive type such as `u32`, `string`, etc
        let child = node.child(0)?;
        let text = doc.text().byte_slice(child.byte_range());
        let (_, header, desc) = find_primitive([text])?;

        Some(Hover {
            range: Some(ts_range_to_lsp_range(node.range())),
            contents: HoverContents::Scalar(MarkedString::String(format!(
                "# {header}\n\n{desc}\n"
            ))),
        })
    } else {
        // May be a referenced type that needs to be resolved,
        // if we are hovering over a qualified / namespaced type
        // we should also make sure to resolve the *full* reference
        let node = match node.parent() {
            Some(p) if p.kind() == "namespaced_type" => p,
            _ => node,
        };

        let typ = ReferencedType::from_node(node)?;
        let decl = typ.resolve_declaration(doc)?;

        // We show an auto-formatted version of the type declaration
        // here to automatically de-indent and make it easier to read
        let text = doc.text_bytes();
        let config = zap_formatter::Config::new(text.as_slice());

        let mut formatted = String::new();
        if zap_formatter::format_root(&mut formatted, config, *decl.as_ref()).is_err() {
            formatted = decl.declaration_text(doc);
        }

        Some(Hover {
            range: Some(ts_range_to_lsp_range(typ.reference_range())),
            contents: HoverContents::Scalar(MarkedString::String(format!(
                "```zap\n{formatted}\n```",
            ))),
        })
    }
}
