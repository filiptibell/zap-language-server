/**
    The type of indentation to use while formatting.
*/
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Indentation {
    /// A single tab.
    #[default]
    Tabs,
    /// Four spaces.
    Spaces,
}

impl Indentation {
    #[must_use]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Tabs => "\t",
            Self::Spaces => "    ",
        }
    }
}
