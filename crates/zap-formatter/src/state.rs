use tree_sitter::Node;

use crate::config::Config;

pub(crate) struct State<'a> {
    pub(crate) config: Config<'a>,
    pub(crate) depth: u32,
}

impl<'a> State<'a> {
    pub(crate) fn new(config: Config<'a>, initial_depth: u32) -> Self {
        State {
            config,
            depth: initial_depth,
        }
    }

    pub(crate) fn text(&self, node: Node) -> &str {
        node.utf8_text(self.config.source).unwrap_or_default()
    }

    pub(crate) fn indent(&self) -> String {
        self.config.indentation.as_str().repeat(self.depth as usize)
    }

    pub(crate) fn increase_depth(&mut self) {
        self.depth = self.depth.saturating_add(1);
    }

    pub(crate) fn decrease_depth(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }
}
