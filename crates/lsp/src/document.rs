use async_lsp::lsp_types::Url;
use ropey::Rope;

/**
    A document tracked by the language server, containing
    the URL, text, and language of the document.

    Not meant to be updated by external sources, only read,
    since the language server should be responsible for
    always keeping the document up-to-date when edits occur.
*/
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
        Returns the language of the document.
    */
    #[must_use]
    pub fn lang(&self) -> &str {
        &self.lang
    }
}
