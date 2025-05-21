use std::fmt;

use tree_sitter::Node;

use crate::{result::Result, state::State};

pub(crate) fn format_range(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    if node.kind() == "range" {
        // Ranges have a single inner variant
        let inner = node.child(0).expect("valid range");
        format_range(writer, state, inner)?;
    } else {
        // We are inside an inner range
        write!(writer, "(")?;

        let mut cursor = node.walk();
        let mut contents = Vec::new();
        for child in node.children(&mut cursor) {
            if matches!(child.kind(), ".." | "number" | "identifier") {
                contents.push(state.text(child));
            }
        }

        for text in contents {
            write!(writer, "{text}")?;
        }

        write!(writer, ")")?;
    }

    Ok(())
}
