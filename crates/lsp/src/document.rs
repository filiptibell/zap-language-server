use std::io::{Read, Result, Write};

use async_lsp::lsp_types::Url;
use ropey::Rope;

/**
    A document tracked by the language server, containing
    the URL, text, and language identifier of the document.

    May be cloned somewhat cheaply to take a snapshot
    of the current state of the document.

    Not meant to be updated by external sources, only read,
    since the language server should be responsible for
    always keeping the document up-to-date when edits occur.
*/
#[derive(Debug, Clone)]
pub struct Document {
    pub(crate) uri: Url,
    pub(crate) text: Rope,
    pub(crate) lang: String,
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
        Returns the language identifier of the document.
    */
    #[must_use]
    pub fn lang(&self) -> &str {
        &self.lang
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
