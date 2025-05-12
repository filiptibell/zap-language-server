#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::items_after_statements)]

use std::io::{Read, Result, Write};

use async_lsp::lsp_types::Url;
use ropey::Rope;

#[cfg(feature = "tree-sitter")]
use async_lsp::lsp_types::{Position, Range};

#[cfg(feature = "tree-sitter")]
use tree_sitter::{Language, Node, Tree};

/**
    A document tracked by the language server, containing
    the URL, text, version, and language of the document.

    May be cloned somewhat cheaply to take a snapshot
    of the current state of the document.

    Not meant to be updated by external sources, only read,
    since the language server should be responsible for
    always keeping the document up-to-date when edits occur.

    # `tree-sitter`

    With the `tree-sitter` crate feature enabled, the document
    may also optionally store a [`tree_sitter::Language`] and
    a parsed [`tree_sitter::Tree`] for the document's text.

    If a `tree-sitter` language has been associated with the
    document, the respective tree will be parsed using the initial
    contents, and incrementally updated thereafter, transparently.
*/
#[derive(Debug, Clone)]
pub struct Document {
    pub(crate) uri: Url,
    pub(crate) text: Rope,
    pub(crate) version: i32,
    pub(crate) language: String,
    #[cfg(feature = "tree-sitter")]
    pub(crate) tree_sitter_lang: Option<Language>,
    #[cfg(feature = "tree-sitter")]
    pub(crate) tree_sitter_tree: Option<Tree>,
}

impl Document {
    /**
        Returns the URL of the document.
    */
    #[must_use]
    pub fn url(&self) -> &Url {
        &self.uri
    }

    /**
        Returns the text of the document, as
        its underlying [`Rope`] representation.

        It is usually easier to use one of the several convenience
        methods that [`Document`] provides for accessing and searching
        through text, but this method exists as an escape hatch.
    */
    #[must_use]
    pub fn text(&self) -> &Rope {
        &self.text
    }

    /**
        Returns a reader over the full text in the document.
    */
    #[must_use]
    pub fn text_reader(&self) -> DocumentReader {
        DocumentReader {
            chunks: self.text.chunks(),
        }
    }

    /**
        Returns the full text of the document, as a string.

        When possible, prefer [`Document::text_reader`]
        for improved performance and less allocations.
    */
    #[must_use]
    pub fn text_contents(&self) -> String {
        self.text.to_string()
    }

    /**
        Returns the full text of the document, as a string.

        When possible, prefer [`Document::text_reader`]
        for improved performance and less allocations.
    */
    #[must_use]
    pub fn text_bytes(&self) -> Vec<u8> {
        self.text.bytes().collect()
    }

    /**
        Returns the version of the document.

        This number should be strictly increasing with
        each change to the document, including undo/redo.
    */
    #[must_use]
    pub fn version(&self) -> i32 {
        self.version
    }

    /**
        Returns the language of the document.
    */
    #[must_use]
    pub fn language(&self) -> &str {
        &self.language
    }
}

#[cfg(feature = "tree-sitter")]
impl Document {
    /**
        Returns `true` if the document has an assigned tree-sitter language, otherwise `false`.
    */
    #[must_use]
    pub fn has_syntax_language(&self) -> bool {
        self.tree_sitter_lang.is_some()
    }

    /**
        Returns `true` if the document has a parsed tree-sitter syntax tree, otherwise `false`.
    */
    #[must_use]
    pub fn has_syntax_tree(&self) -> bool {
        self.tree_sitter_tree.is_some()
    }

    /**
        Returns a [`Node`] at the root of the syntax tree, if one exists.
    */
    #[must_use]
    pub fn node_at_root(&self) -> Option<Node> {
        self.tree_sitter_tree.as_ref().map(|tree| tree.root_node())
    }

    /**
        Returns a [`Node`] at the given position, if one exists.
    */
    #[must_use]
    pub fn node_at_position(&self, position: Position) -> Option<Node> {
        let tree = self.tree_sitter_tree.as_ref()?;

        let point = tree_sitter::Point {
            row: position.line as usize,
            column: position.character as usize,
        };

        tree.root_node()
            .named_descendant_for_point_range(point, point)
    }

    /**
        Returns a triple of three [`Node`] at the given position, if they exist.

        The returned nodes will be the following:

        - The top-level node, which is an immediate child of the tree root
        - The parent of the node at the position
        - The node at the position
    */
    #[must_use]
    pub fn node_triple_at_position(&self, pos: Position) -> Option<(Node, Node, Node)> {
        let root = self.node_at_root()?;
        let node = self.node_at_position(pos)?;
        let parent = node.parent()?;

        let mut top_level = parent;
        while let Some(parent) = top_level.parent() {
            if parent == root {
                break;
            } else {
                top_level = parent;
            }
        }

        if top_level == parent {
            return None;
        }

        Some((top_level, parent, node))
    }

    /**
        Creates and runs a query for the given query string.

        Returns `Some(captures)` if the query was successful, otherwise `None`.
    */
    #[must_use]
    pub fn query(&self, query: impl AsRef<str>) -> Option<Vec<DocumentQueryCapture>> {
        use tree_sitter::{Node, Point, Query, QueryCursor, StreamingIterator};

        let lang = self.tree_sitter_lang.as_ref()?;
        let tree = self.tree_sitter_tree.as_ref()?;

        let query = Query::new(lang, query.as_ref()).ok()?;
        let query_names = query.capture_names();

        let doc_text = self.text.to_string();
        let doc_bytes = doc_text.as_bytes();

        let mut cursor = QueryCursor::new();
        let mut it = cursor.matches(&query, tree.root_node(), doc_bytes);

        fn point_to_position(point: Point) -> Position {
            Position {
                line: point.row as u32,
                character: point.column as u32,
            }
        }

        fn range_from_node(node: &Node) -> Range {
            Range {
                start: point_to_position(node.start_position()),
                end: point_to_position(node.end_position()),
            }
        }

        let mut items = Vec::new();
        while let Some(matched) = it.next() {
            for capture in matched.captures {
                if let Ok(text) = capture.node.utf8_text(doc_bytes) {
                    let name = query_names[capture.index as usize].to_string();
                    let text = text.to_string();
                    let range = range_from_node(&capture.node);
                    items.push(DocumentQueryCapture { name, text, range });
                }
            }
        }
        Some(items)
    }
}

/**
    A reader over the full text contents of a document.

    Created by calling [`Document::text_reader`].
*/
pub struct DocumentReader<'d> {
    chunks: ropey::iter::Chunks<'d>,
}

impl Read for DocumentReader<'_> {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        match self.chunks.next() {
            Some(chunk) => buf.write(chunk.as_bytes()),
            _ => Ok(0),
        }
    }
}

#[cfg(feature = "tree-sitter")]
/**
    A capture from a tree-sitter query on a document.

    Created by calling [`Document::query`].
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentQueryCapture {
    /// The capture name
    pub name: String,
    /// The textual contents of the capture
    pub text: String,
    /// The document range of the capture
    pub range: Range,
}
