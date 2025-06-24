use async_language_server::{
    server::Document,
    tree_sitter::{Node, Range},
    tree_sitter_utils::find_ancestor,
};

use super::{DeclaredType, is_namespace};
use crate::utils::is_type_reference;

/**
    Represents a referenced type in a source file tree.

    Not to be confused with [`DeclaredType`].

    Namespace fields represent the dot-separated
    namespace path to the final type identifier.

    ### Example

    ```zap
    type Ref = A.B.C
    ```
*/
#[derive(Debug, Clone)]
pub struct ReferencedType<'doc: 'tree, 'tree> {
    pub(super) document: &'doc Document,
    /// `A.B.C`
    pub(super) reference: Node<'tree>,
    /// `A`, `B`
    pub(super) namespaces: Vec<Node<'tree>>,
    /// `C`
    pub(super) identifier: Node<'tree>,
}

impl<'doc: 'tree, 'tree> ReferencedType<'doc, 'tree> {
    /**
        Constructs a new `ReferencedType` from a given node,
        if the node is a valid type reference.
    */
    pub fn from_node(doc: &'doc Document, node: Node<'tree>) -> Option<Self> {
        if node.kind() == "namespaced_type" {
            Some(Self {
                document: doc,
                reference: node,
                namespaces: node
                    .children_by_field_name("namespace", &mut node.walk())
                    .collect(),
                identifier: node.child_by_field_name("type")?,
            })
        } else if is_type_reference(node) {
            Some(Self {
                document: doc,
                reference: node,
                namespaces: Vec::new(),
                identifier: node,
            })
        } else {
            None
        }
    }

    /**
        Finds all type references in the given document,
        within the given subtree / node.
    */
    pub fn find_all_in(document: &'doc Document, node: Node<'tree>) -> Vec<Self> {
        fn recurse<'a: 'b, 'b>(
            document: &'a Document,
            current_node: Node<'b>,
            results: &mut Vec<ReferencedType<'a, 'b>>,
        ) {
            match current_node.kind() {
                "namespaced_type" | "identifier" => {
                    results.extend(ReferencedType::from_node(document, current_node));
                }
                _ => {
                    for child in current_node.children(&mut current_node.walk()) {
                        recurse(document, child, results);
                    }
                }
            }
        }

        let mut results = Vec::new();
        recurse(document, node, &mut results);
        results
    }

    /**
        Finds all type references in the given document.
    */
    pub fn find_all(doc: &'doc Document) -> Vec<Self> {
        doc.node_at_root()
            .map(|n| Self::find_all_in(doc, n))
            .unwrap_or_default()
    }

    /**
        Returns the full node range for this type reference.
    */
    pub fn reference_range(&self) -> Range {
        self.reference.range()
    }

    /**
        Returns the full node text for this type reference.
    */
    pub fn reference_text(&self) -> String {
        self.document.node_text(self.reference)
    }

    /**
        Returns the identifier range for this type reference.
    */
    pub fn identifier_range(&self) -> Range {
        self.identifier.range()
    }

    /**
        Returns the identifier text for this type reference.
    */
    pub fn identifier_text(&self) -> String {
        self.document.node_text(self.identifier)
    }

    /**
        Returns `true` if this reference is a part
        of the given namespace, `false` otherwise.
    */
    pub fn is_in_namespace(&self, namespace: Node) -> bool {
        find_ancestor(self.reference, is_namespace).is_some_and(|found| found == namespace)
    }

    /**
        Finds the declaration, if any, for this type reference.
    */
    pub fn resolve_declaration(&self) -> Option<DeclaredType<'doc, 'tree>> {
        // 1. First, we must find the correct root namespace to search in
        let mut namespace = find_ancestor(self.reference, is_namespace)?;

        // 2. Next, if our type reference contains namespace identifiers,
        //    we should walk all of those, or return None if any is missing
        'outer: for &ident_node in &self.namespaces {
            let ident_text = self.document.node_text(ident_node);

            let mut cursor = namespace.walk();
            for child in namespace.children(&mut cursor) {
                if child.kind() == "namespace_declaration" {
                    let name_node = child
                        .child_by_field_name("name")
                        .expect("valid namespace declaration");
                    let name_text = self.document.node_text(name_node);
                    if name_text == ident_text {
                        namespace = child;
                        continue 'outer;
                    }
                }
            }

            return None;
        }

        // 3. We should now be in the correct namespace, find the type declaration
        let ident_referenced = self.identifier_text();

        let mut cursor = namespace.walk();
        for child in namespace.children(&mut cursor) {
            if child.kind() == "type_declaration" {
                let name_node = child
                    .child_by_field_name("name")
                    .expect("valid type declaration");
                let name_text = self.document.node_text(name_node);
                if name_text == ident_referenced {
                    let res = DeclaredType::from_node(self.document, child);
                    return res;
                }
            }
        }

        None
    }
}

impl<'tree> AsRef<Node<'tree>> for ReferencedType<'_, 'tree> {
    fn as_ref(&self) -> &Node<'tree> {
        &self.reference
    }
}
