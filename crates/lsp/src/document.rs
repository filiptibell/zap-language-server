use std::io::{Read, Result, Write};

use async_lsp::lsp_types::Url;
use ropey::Rope;

#[cfg(feature = "tree-sitter")]
use tree_sitter::{Language, Parser, Tree};

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

    /**
        Creates a parser with the tree-sitter language
        for the document pre-assigned to it.
    */
    #[must_use]
    pub(crate) fn parser(&self) -> Option<Parser> {
        let lang = self.tree_sitter_lang.clone()?;
        let mut parser = Parser::new();
        if parser.set_language(&lang).is_ok() {
            Some(parser)
        } else {
            None
        }
    }

    /**
        Returns the parsed tree for the document, if any.
    */
    #[must_use]
    pub fn tree(&self) -> Option<&Tree> {
        self.tree_sitter_tree.as_ref()
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
