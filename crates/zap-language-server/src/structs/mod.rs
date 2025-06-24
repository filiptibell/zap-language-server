#![allow(dead_code)]

use zap_language::tree_sitter::Node;

mod declared_namespace;
mod declared_type;
mod referenced_namespace;
mod referenced_type;

pub use self::declared_namespace::DeclaredNamespace;
pub use self::declared_type::DeclaredType;
pub use self::referenced_namespace::ReferencedNamespace;
pub use self::referenced_type::ReferencedType;

fn is_namespace(node: Node) -> bool {
    matches!(node.kind(), "source_file" | "namespace_declaration")
}
