use std::fmt;

use tree_sitter::Node;

use crate::{result::Result, state::State};

pub(crate) fn format_array(writer: &mut impl fmt::Write, state: &mut State, node: Node) -> Result {
    if node.kind() == "array" {
        // Arrays have a single inner variant
        let inner = node.child(0).expect("valid array");
        format_array(writer, state, inner)?;
    } else {
        // We are inside an inner array
        write!(writer, "[")?;

        let mut cursor = node.walk();
        let mut contents = Vec::new();
        for child in node.children(&mut cursor) {
            if matches!(child.kind(), "number" | "identifier") {
                contents.push(state.text(child));
            }
        }

        for (index, text) in contents.iter().enumerate() {
            if index > 0 {
                write!(writer, "..")?;
            }
            write!(writer, "{text}")?;
        }

        write!(writer, "]")?;
    }

    Ok(())
}
