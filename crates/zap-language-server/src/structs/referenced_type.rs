use async_language_server::{
    server::Document,
    tree_sitter::{Node, Range},
    tree_sitter_utils::find_ancestor,
};

use crate::utils::{is_namespace, is_type_reference};

use super::DeclaredType;

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
pub struct ReferencedType<'a> {
    /// `A.B.C`
    pub(super) reference: Node<'a>,
    /// `A`, `B`
    pub(super) namespaces: Vec<Node<'a>>,
    /// `C`
    pub(super) identifier: Node<'a>,
}

impl<'a> ReferencedType<'a> {
    /**
        Constructs a new `ReferencedType` from a given node,
        if the node is a valid type reference.
    */
    pub fn from_node(node: Node<'a>) -> Option<Self> {
        if node.kind() == "namespaced_type" {
            Some(Self {
                reference: node,
                namespaces: node
                    .children_by_field_name("namespace", &mut node.walk())
                    .collect(),
                identifier: node.child_by_field_name("type")?,
            })
        } else if is_type_reference(node) {
            Some(Self {
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
    pub fn find_all_in(node: Node<'a>) -> Vec<Self> {
        fn recurse<'b>(current_node: Node<'b>, results: &mut Vec<ReferencedType<'b>>) {
            match current_node.kind() {
                "namespaced_type" | "identifier" => {
                    results.extend(ReferencedType::from_node(current_node));
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
        Finds all type references in the given document.
    */
    pub fn find_all(doc: &'a Document) -> Vec<Self> {
        doc.node_at_root()
            .map(Self::find_all_in)
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
    pub fn reference_text(&self, doc: &Document) -> String {
        doc.node_text(self.reference)
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
        Finds the final namespace, if any, for this type reference.

        An optional `limit` may be provided to only walk a certain number of namespaces.
    */
    pub fn resolve_namespace<'d: 'a>(
        &self,
        doc: &'d Document,
        limit: Option<usize>,
    ) -> Option<Node<'a>> {
        let mut namespace = find_ancestor(self.reference, is_namespace)?;
        let mut current = 0;

        'outer: for &ident_node in &self.namespaces {
            if limit.is_some_and(|l| current >= l) {
                break;
            }

            let ident_text = doc.node_text(ident_node);

            let mut cursor = namespace.walk();
            for child in namespace.children(&mut cursor) {
                if child.kind() == "namespace_declaration" {
                    let name_node = child
                        .child_by_field_name("name")
                        .expect("valid namespace declaration");
                    let name_text = doc.node_text(name_node);
                    if name_text == ident_text {
                        namespace = child;
                        current += 1;
                        continue 'outer;
                    }
                }
            }

            return None;
        }

        Some(namespace)
    }

    /**
        Finds the declaration, if any, for this type reference.
    */
    pub fn resolve_declaration<'d: 'a>(&self, doc: &'d Document) -> Option<DeclaredType<'a>> {
        let namespace = self.resolve_namespace(doc, None)?;

        let ident_referenced = self.identifier_text(doc);

        let mut cursor = namespace.walk();
        for child in namespace.children(&mut cursor) {
            if child.kind() == "type_declaration" {
                let name_node = child
                    .child_by_field_name("name")
                    .expect("valid type declaration");
                let name_text = doc.node_text(name_node);
                if name_text == ident_referenced {
                    let res = DeclaredType::from_node(child);
                    return res;
                }
            }
        }

        None
    }
}

impl<'a> AsRef<Node<'a>> for ReferencedType<'a> {
    fn as_ref(&self) -> &Node<'a> {
        &self.reference
    }
}
