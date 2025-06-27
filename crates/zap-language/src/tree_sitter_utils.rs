use tree_sitter::Node;

#[must_use]
pub fn is_known_node(node: Node) -> bool {
    is_comment_node(node)
        || is_declaration_node(node)
        || is_type_node(node)
        || is_tuple_node(node)
        || is_range_node(node)
        || is_array_node(node)
        || is_ident_node(node)
}

#[must_use]
pub fn is_comment_node(node: Node) -> bool {
    matches!(node.kind(), "comment" | "doc_comment")
}

#[must_use]
pub fn is_declaration_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "option_declaration"
            | "type_declaration"
            | "event_declaration"
            | "function_declaration"
            | "namespace_declaration"
    )
}

#[must_use]
pub fn is_type_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "type"
            | "namespaced_type"
            | "primitive_type"
            | "optional_type"
            | "struct_type"
            | "enum_type"
            | "map_type"
            | "set_type"
    )
}

#[must_use]
pub fn is_tuple_node(node: Node) -> bool {
    matches!(node.kind(), "tuple" | "tuple_value")
}

#[must_use]
pub fn is_range_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "range" | "range_empty" | "range_exact" | "range_inexact"
    )
}

#[must_use]
pub fn is_array_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "array" | "array_empty" | "array_exact" | "array_inexact"
    )
}

#[must_use]
pub fn is_ident_node(node: Node) -> bool {
    matches!(node.kind(), "identifier")
}

#[must_use]
pub fn is_field_node(node: Node) -> bool {
    let kind = node.kind();

    let is_event = kind.starts_with("event_");
    let is_funct = kind.starts_with("function_");
    let is_field = kind.ends_with("_field");

    (is_event || is_funct) && is_field
}

#[must_use]
pub fn is_field_value_node(node: Node) -> bool {
    let kind = node.kind();

    let is_event = kind.starts_with("event_");
    let is_funct = kind.starts_with("function_");
    let is_value = kind.ends_with("_value");

    (is_event || is_funct) && is_value
}

#[must_use]
pub fn is_punctuation(c: char) -> bool {
    matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | ':' | ',' | '.')
}

#[must_use]
pub fn is_punctuation_str(s: impl AsRef<str>) -> bool {
    s.as_ref().chars().all(is_punctuation)
}

#[must_use]
pub fn is_punctuation_node(node: Node) -> bool {
    is_punctuation_str(node.kind())
}

#[must_use]
pub fn is_atom(node: Node) -> bool {
    node.child_count() == 0
}

#[must_use]
pub fn is_type_empty(node: Node, skip: Option<usize>) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor).skip(skip.unwrap_or_default()) {
        if matches!(
            child.kind(),
            "property" | "identifier" | "enum_variant" | "tuple_value"
        ) {
            return false;
        }
    }
    true
}

/**
    An iterator over **all** descendant nodes, in depth-first (positional) order.

    Includes both top-level items such as `namespace_declaration`, `type_declaration`,
    and others, as well as atoms such as punctuation, and plain text.
*/
#[derive(Debug, Clone)]
pub struct DepthFirstNodeIterator<'a> {
    queue: Vec<Node<'a>>,
}

impl<'a> DepthFirstNodeIterator<'a> {
    #[must_use]
    pub fn new(root: Node<'a>) -> Self {
        Self::from(root)
    }
}

impl<'a> From<Node<'a>> for DepthFirstNodeIterator<'a> {
    fn from(root: Node<'a>) -> Self {
        Self { queue: vec![root] }
    }
}

impl<'a> Iterator for DepthFirstNodeIterator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.queue.pop() {
            let mut cursor = node.walk();

            let children = node.children(&mut cursor).collect::<Vec<_>>();
            for child in children.into_iter().rev() {
                self.queue.push(child);
            }

            Some(node)
        } else {
            None
        }
    }
}

/**
    An iterator over **atoms**, in depth-first (positional) order.

    Does *not* include top-level items such as `namespace_declaration`,
    `type_declaration`, only atoms such as punctuation, and plain text.
*/
#[derive(Debug, Clone)]
pub struct AtomIterator<'a> {
    inner: DepthFirstNodeIterator<'a>,
}

impl<'a> AtomIterator<'a> {
    #[must_use]
    pub fn new(root: Node<'a>) -> Self {
        Self::from(root)
    }
}

impl<'a> From<Node<'a>> for AtomIterator<'a> {
    fn from(root: Node<'a>) -> Self {
        Self {
            inner: DepthFirstNodeIterator::from(root),
        }
    }
}

impl<'a> Iterator for AtomIterator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.by_ref().find(|&node| is_atom(node))
    }
}
