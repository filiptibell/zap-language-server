use std::fmt;

use tree_sitter::Node;

use crate::{result::Result, state::State};

pub(crate) fn format_comment(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    let text = state.text(node);
    let text = format_comment_contents(text);
    write!(writer, "{text}")?; // No space before comment
    Ok(())
}

pub(crate) fn format_inline_comment(
    writer: &mut impl fmt::Write,
    state: &mut State,
    node: Node,
) -> Result {
    let text = state.text(node);
    let text = format_comment_contents(text);
    write!(writer, " {text}")?; // Space before comment
    Ok(())
}

fn format_comment_contents(comment: &str) -> String {
    if comment.starts_with("--") && comment.len() > 2 {
        // Normal comment, might be missing leading whitespace
        if comment.chars().nth(2).is_none_or(|c| !c.is_whitespace()) {
            return format!("-- {}", &comment[2..]);
        }
    } else if comment.starts_with("---") && comment.len() > 3 {
        // Doc comment, might be missing leading whitespace
        if comment.chars().nth(3).is_none_or(|c| !c.is_whitespace()) {
            return format!("--- {}", &comment[3..]);
        }
    }
    comment.to_string()
}
