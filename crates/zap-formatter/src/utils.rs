use tree_sitter::Node;

#[derive(Debug, Clone)]
pub(crate) struct DepthFirstNodeIterator<'a> {
    queue: Vec<Node<'a>>,
}

impl<'a> DepthFirstNodeIterator<'a> {
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

pub(crate) fn is_type_empty(node: Node) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if matches!(child.kind(), "property" | "identifier" | "enum_variant") {
            return false;
        }
    }
    true
}
