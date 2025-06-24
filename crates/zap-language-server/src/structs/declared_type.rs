use async_language_server::{
    server::Document,
    tree_sitter::{Node, Range},
    tree_sitter_utils::find_ancestor,
};

use super::{ReferencedType, is_namespace};

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
pub struct DeclaredType<'doc: 'tree, 'tree> {
    pub(super) document: &'doc Document,
    /// `type C = u32`
    pub(super) declaration: Node<'tree>,
    /// `namespace A = { ... }`, `namespace B = { ... }`
    pub(super) namespaces: Vec<Node<'tree>>,
    /// `C`
    pub(super) identifier: Node<'tree>,
}

impl<'doc: 'tree, 'tree> DeclaredType<'doc, 'tree> {
    /**
        Constructs a new `DeclaredType` from a given node,
        if the node is a valid type reference.
    */
    pub fn from_node(doc: &'doc Document, node: Node<'tree>) -> Option<Self> {
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
                document: doc,
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
    pub fn find_all_in(doc: &'doc Document, node: Node<'tree>) -> Vec<Self> {
        fn recurse<'a: 'b, 'b>(
            document: &'a Document,
            current_node: Node<'b>,
            results: &mut Vec<DeclaredType<'a, 'b>>,
        ) {
            match current_node.kind() {
                "source_file" | "namespace_declaration" => {
                    for child in current_node.children(&mut current_node.walk()) {
                        recurse(document, child, results);
                    }
                }
                "type_declaration" => {
                    results.extend(DeclaredType::from_node(document, current_node));
                }
                _ => {}
            }
        }

        let mut results = Vec::new();
        recurse(doc, node, &mut results);
        results
    }

    /**
        Finds all type declarations in the given document.
    */
    pub fn find_all(doc: &'doc Document) -> Vec<Self> {
        doc.node_at_root()
            .map(|n| Self::find_all_in(doc, n))
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
    pub fn declaration_text(&self) -> String {
        self.document.node_text(self.declaration)
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
    pub fn identifier_text(&self) -> String {
        self.document.node_text(self.identifier)
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
    pub fn resolve_references(&self) -> Vec<ReferencedType<'doc, 'tree>> {
        let decl_ident = self.document.node_text(self.identifier);

        ReferencedType::find_all(self.document)
            .into_iter()
            // First pass - filter by same final identifier (fast)
            .filter(|referenced| {
                let ref_ident = referenced.document.node_text(referenced.identifier);
                ref_ident == decl_ident
            })
            // Second pass - resolve the full declaration (potentially slow)
            .filter(|referenced| {
                referenced
                    .resolve_declaration()
                    .is_some_and(|other| self.declaration == other.declaration)
            })
            .collect()
    }
}

impl<'tree> AsRef<Node<'tree>> for DeclaredType<'_, 'tree> {
    fn as_ref(&self) -> &Node<'tree> {
        &self.declaration
    }
}
