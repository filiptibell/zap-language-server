use std::fmt;

use tree_sitter::Node;
use zap_language::docs::is_punctuation_str;

use crate::{result::Result, state::State, utils::DepthFirstNodeIterator};

pub(crate) fn format_unknown(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    let mut had_first = false;
    let mut last_punc = false;
    for node in DepthFirstNodeIterator::new(node).filter(|n| n.child_count() == 0) {
        let text = state.text(node);
        if had_first {
            // Only write sep spacing if not near punctuation
            let is_punc = is_punctuation_str(text);
            if !is_punc && !last_punc {
                write!(writer, " ")?;
            }
            last_punc = is_punc;
        } else {
            had_first = true;
        }
        write!(writer, "{text}")?;
    }

    Ok(())
}
