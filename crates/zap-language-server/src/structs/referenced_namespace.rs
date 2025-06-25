use async_language_server::{
    server::Document,
    tree_sitter::{Node, Range},
    tree_sitter_utils::find_ancestor,
};

use crate::utils::is_namespace;

use super::DeclaredNamespace;

/**
    Represents a referenced namespace in a source file tree.

    Not to be confused with [`DeclaredNamespace`].

    This represents namespace identifiers used in namespaced type references.

    ### Example

    ```zap
    type Ref = A.B.C
    ```

    Here, `A` and `B` are namespace references.
*/
#[derive(Debug, Clone)]
pub struct ReferencedNamespace<'a> {
    /// `A.B.C` - the full namespaced type
    pub(super) reference: Node<'a>,
    /// `A` (for namespace B reference) - parent namespace nodes
    pub(super) namespaces: Vec<Node<'a>>,
    /// `B` - the specific namespace identifier being referenced
    pub(super) identifier: Node<'a>,
}

impl<'a> ReferencedNamespace<'a> {
    /**
        Constructs a new `ReferencedNamespace` from a given node,
        if the node is a valid namespace reference.
    */
    pub fn from_node(node: Node<'a>) -> Option<Self> {
        // Check if this identifier is part of a namespaced type
        let namespaced_type = find_ancestor(node, |n| n.kind() == "namespaced_type")?;

        // Get all namespace identifiers from the namespaced type
        let all_namespaces: Vec<Node> = namespaced_type
            .children_by_field_name("namespace", &mut namespaced_type.walk())
            .collect();

        // Find the position of our node in the namespace chain
        let node_position = all_namespaces.iter().position(|&n| n == node)?;

        // Parent namespaces are all the namespaces before this one
        let parent_namespaces = all_namespaces[..node_position].to_vec();

        Some(Self {
            reference: namespaced_type,
            namespaces: parent_namespaces,
            identifier: node,
        })
    }

    /**
        Finds all namespace references in the given document,
        within the given subtree / node.
    */
    pub fn find_all_in(node: Node<'a>) -> Vec<Self> {
        fn recurse<'b>(current_node: Node<'b>, results: &mut Vec<ReferencedNamespace<'b>>) {
            match current_node.kind() {
                "namespaced_type" => {
                    // Add all namespace identifiers from this namespaced type
                    for namespace_node in
                        current_node.children_by_field_name("namespace", &mut current_node.walk())
                    {
                        results.extend(ReferencedNamespace::from_node(namespace_node));
                    }
                }
                _ => {
                    for child in current_node.children(&mut current_node.walk()) {
                        recurse(child, results);
                    }
                }
            }
        }

        let mut results = Vec::new();
        recurse(node, &mut results);
        results
    }

    /**
        Finds all namespace references in the given document.
    */
    pub fn find_all(doc: &'a Document) -> Vec<Self> {
        doc.node_at_root()
            .map(Self::find_all_in)
            .unwrap_or_default()
    }

    /**
        Returns the full node range for this namespace reference.
    */
    pub fn reference_range(&self) -> Range {
        self.reference.range()
    }

    /**
        Returns the full node text for this namespace reference.
    */
    pub fn reference_text(&self, doc: &Document) -> String {
        doc.node_text(self.reference)
    }

    /**
        Returns the identifier range for this namespace reference.
    */
    pub fn identifier_range(&self) -> Range {
        self.identifier.range()
    }

    /**
        Returns the identifier text for this namespace reference.
    */
    pub fn identifier_text(&self, doc: &Document) -> String {
        doc.node_text(self.identifier)
    }

    /**
        Returns `true` if this reference is a part
        of the given namespace, `false` otherwise.
    */
    pub fn is_in_namespace(&self, namespace: Node) -> bool {
        find_ancestor(self.reference, is_namespace).is_some_and(|found| found == namespace)
    }

    /**
        Finds the declaration, if any, for this namespace reference.
    */
    pub fn resolve_declaration<'d: 'a>(&self, doc: &'d Document) -> Option<DeclaredNamespace<'a>> {
        // 1. First, we must find the correct root namespace to search in
        let mut namespace = find_ancestor(self.reference, is_namespace)?;

        // 2. Next, if our namespace reference has parent namespaces,
        //    we should walk all of those, or return None if any is missing
        'outer: for &parent_ident_node in &self.namespaces {
            let parent_ident_text = doc.node_text(parent_ident_node);

            let mut cursor = namespace.walk();
            for child in namespace.children(&mut cursor) {
                if child.kind() == "namespace_declaration" {
                    let name_node = child
                        .child_by_field_name("name")
                        .expect("valid namespace declaration");
                    let name_text = doc.node_text(name_node);
                    if name_text == parent_ident_text {
                        namespace = child;
                        continue 'outer;
                    }
                }
            }

            return None;
        }

        // 3. We should now be in the correct namespace, find the namespace declaration
        let ident_referenced = self.identifier_text(doc);

        let mut cursor = namespace.walk();
        for child in namespace.children(&mut cursor) {
            if child.kind() == "namespace_declaration" {
                let name_node = child
                    .child_by_field_name("name")
                    .expect("valid namespace declaration");
                let name_text = doc.node_text(name_node);
                if name_text == ident_referenced {
                    return DeclaredNamespace::from_node(child);
                }
            }
        }

        None
    }
}

impl<'a> AsRef<Node<'a>> for ReferencedNamespace<'a> {
    fn as_ref(&self) -> &Node<'a> {
        &self.reference
    }
}
