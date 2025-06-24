#![allow(dead_code)]

use zap_language::tree_sitter::Node;

mod declared_type;
mod referenced_type;

pub use self::declared_type::DeclaredType;
pub use self::referenced_type::ReferencedType;

fn is_namespace(node: Node) -> bool {
    matches!(node.kind(), "source_file" | "namespace_declaration")
}
