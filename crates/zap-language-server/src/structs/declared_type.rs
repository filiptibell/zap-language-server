use async_language_server::{
    server::Document,
    tree_sitter::{Node, Range},
    tree_sitter_utils::find_ancestor,
};

use crate::utils::is_namespace;

use super::ReferencedType;

/**
    Represents a declared type in a source file tree.

    Not to be confused with [`ReferencedType`].

    Namespace fields represent the hierarchy of namespaces,
    with the outermost namespace being the first element.

    ### Example

    ```zap
    namespace A = {
        namespace B = {
            type C = u32
        }
    }
    ```
*/
#[derive(Debug, Clone)]
pub struct DeclaredType<'a> {
    /// `type C = u32`
    pub(super) declaration: Node<'a>,
    /// `namespace A = { ... }`, `namespace B = { ... }`
    pub(super) namespaces: Vec<Node<'a>>,
    /// `C`
    pub(super) identifier: Node<'a>,
}

impl<'a> DeclaredType<'a> {
    /**
        Constructs a new `DeclaredType` from a given node,
        if the node is a valid type reference.
    */
    pub fn from_node(node: Node<'a>) -> Option<Self> {
        if node.kind() == "type_declaration" {
            let mut namespaces = Vec::new();
            let mut namespace = node.parent();

            while let Some(parent) = namespace {
                if parent.kind() == "namespace_declaration" {
                    namespaces.push(parent);
                }
                namespace = parent.parent();
            }

            namespaces.reverse(); // Top-level first

            Some(Self {
                declaration: node,
                namespaces,
                identifier: node.child_by_field_name("name")?,
            })
        } else {
            None
        }
    }

    /**
        Finds all type declarations in the given document,
        within the given subtree / node.
    */
    pub fn find_all_in(node: Node<'a>) -> Vec<Self> {
        fn recurse<'b>(current_node: Node<'b>, results: &mut Vec<DeclaredType<'b>>) {
            match current_node.kind() {
                "source_file" | "namespace_declaration" => {
                    for child in current_node.children(&mut current_node.walk()) {
                        recurse(child, results);
                    }
                }
                "type_declaration" => {
                    results.extend(DeclaredType::from_node(current_node));
                }
                _ => {}
            }
        }

        let mut results = Vec::new();
        recurse(node, &mut results);
        results
    }

    /**
        Finds all type declarations in the given document.
    */
    pub fn find_all(doc: &'a Document) -> Vec<Self> {
        doc.node_at_root()
            .map(Self::find_all_in)
            .unwrap_or_default()
    }

    /**
        Returns the full node range for this type declaration.
    */
    pub fn declaration_range(&self) -> Range {
        self.declaration.range()
    }

    /**
        Returns the full node text for this type declaration.
    */
    pub fn declaration_text(&self, doc: &Document) -> String {
        doc.node_text(self.declaration)
    }

    /**
        Returns the identifier range for this type declaration.
    */
    pub fn identifier_range(&self) -> Range {
        self.identifier.range()
    }

    /**
        Returns the identifier text for this type declaration.
    */
    pub fn identifier_text(&self, doc: &Document) -> String {
        doc.node_text(self.identifier)
    }

    /**
        Returns `true` if this declaration is a part
        of the given namespace, `false` otherwise.
    */
    pub fn is_in_namespace(&self, namespace: Node) -> bool {
        find_ancestor(self.declaration, is_namespace).is_some_and(|found| found == namespace)
    }

    /**
        Resolves all valid references to this type declaration.
    */
    pub fn resolve_references<'d: 'a>(&self, doc: &'d Document) -> Vec<ReferencedType<'a>> {
        let decl_ident = doc.node_text(self.identifier);

        ReferencedType::find_all(doc)
            .into_iter()
            // First pass - filter by same final identifier (fast)
            .filter(|referenced| {
                let ref_ident = doc.node_text(referenced.identifier);
                ref_ident == decl_ident
            })
            // Second pass - resolve the full declaration (potentially slow)
            .filter(|referenced| {
                referenced
                    .resolve_declaration(doc)
                    .is_some_and(|other| self.declaration == other.declaration)
            })
            .collect()
    }
}

impl<'a> AsRef<Node<'a>> for DeclaredType<'a> {
    fn as_ref(&self) -> &Node<'a> {
        &self.declaration
    }
}
