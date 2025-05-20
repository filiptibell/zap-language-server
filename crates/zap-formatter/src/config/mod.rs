mod indentation;

pub use self::indentation::Indentation;

#[derive(Debug, Clone, Copy)]
pub struct Config<'a> {
    pub(crate) source: &'a [u8],
    pub(crate) columns: usize,
    pub(crate) indentation: Indentation,
}

impl<'a> Config<'a> {
    #[must_use]
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = columns;
        self
    }

    #[must_use]
    pub fn with_indentation(mut self, indentation: Indentation) -> Self {
        self.indentation = indentation;
        self
    }
}

impl Default for Config<'_> {
    fn default() -> Self {
        Self {
            source: &[],
            columns: 80,
            indentation: Indentation::default(),
        }
    }
}
