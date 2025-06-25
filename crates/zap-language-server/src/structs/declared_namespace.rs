use async_language_server::{
    server::Document,
    tree_sitter::{Node, Range},
    tree_sitter_utils::find_ancestor,
};

use crate::utils::is_namespace;

use super::ReferencedNamespace;

/**
    Represents a declared namespace in a source file tree.

    Not to be confused with [`ReferencedNamespace`].

    Namespace fields represent the hierarchy of parent namespaces,
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
pub struct DeclaredNamespace<'a> {
    /// `namespace B = { ... }`
    pub(super) declaration: Node<'a>,
    /// `namespace A = { ... }`
    pub(super) namespaces: Vec<Node<'a>>,
    /// `B`
    pub(super) identifier: Node<'a>,
}

impl<'a> DeclaredNamespace<'a> {
    /**
        Constructs a new `DeclaredNamespace` from a given node,
        if the node is a valid namespace declaration.
    */
    pub fn from_node(node: Node<'a>) -> Option<Self> {
        if node.kind() == "namespace_declaration" {
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
        Finds all namespace declarations in the given document,
        within the given subtree / node.
    */
    pub fn find_all_in(node: Node<'a>) -> Vec<Self> {
        fn recurse<'b>(current_node: Node<'b>, results: &mut Vec<DeclaredNamespace<'b>>) {
            match current_node.kind() {
                "source_file" => {
                    for child in current_node.children(&mut current_node.walk()) {
                        recurse(child, results);
                    }
                }
                "namespace_declaration" => {
                    results.extend(DeclaredNamespace::from_node(current_node));
                    // Also recurse into the namespace to find nested namespaces
                    for child in current_node.children(&mut current_node.walk()) {
                        recurse(child, results);
                    }
                }
                _ => {}
            }
        }

        let mut results = Vec::new();
        recurse(node, &mut results);
        results
    }

    /**
        Finds all namespace declarations in the given document.
    */
    pub fn find_all(doc: &'a Document) -> Vec<Self> {
        doc.node_at_root()
            .map(Self::find_all_in)
            .unwrap_or_default()
    }

    /**
        Returns the full node range for this namespace declaration.
    */
    pub fn declaration_range(&self) -> Range {
        self.declaration.range()
    }

    /**
        Returns the full node text for this namespace declaration.
    */
    pub fn declaration_text(&self, doc: &Document) -> String {
        doc.node_text(self.declaration)
    }

    /**
        Returns the identifier range for this namespace declaration.
    */
    pub fn identifier_range(&self) -> Range {
        self.identifier.range()
    }

    /**
        Returns the identifier text for this namespace declaration.
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
        Resolves all valid references to this namespace declaration.
    */
    pub fn resolve_references<'d: 'a>(&self, doc: &'d Document) -> Vec<ReferencedNamespace<'a>> {
        let decl_ident = doc.node_text(self.identifier);

        ReferencedNamespace::find_all(doc)
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

impl<'a> AsRef<Node<'a>> for DeclaredNamespace<'a> {
    fn as_ref(&self) -> &Node<'a> {
        &self.declaration
    }
}
