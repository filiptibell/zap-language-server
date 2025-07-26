use std::fmt;

use zap_language::tree_sitter::Node;

use crate::{basic::plain::format_plain, result::Result, state::State};

pub(crate) fn format_primitive(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    let prim_node = node.child(0).expect("valid primitive");
    let prim_text = state.text(prim_node);

    if let Some(sep_node) = node.child(1) {
        let sep_text = state.text(sep_node);
        if sep_text == "." {
            let spec_text = node.child(2).map(|n| state.text(n)).unwrap_or_default();
            write!(writer, "{prim_text}.{spec_text}")?;
        } else if sep_text == "(" {
            let spec_text = node.child(2).map(|n| state.text(n)).unwrap_or_default();
            write!(writer, "{prim_text}({spec_text})")?;
        } else {
            format_plain(writer, state, node)?;
        }
    } else {
        format_plain(writer, state, node)?;
    }

    Ok(())
}
