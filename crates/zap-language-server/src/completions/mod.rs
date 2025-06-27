use async_language_server::{
    lsp_types::Position, server::Document, tree_sitter::Node,
    tree_sitter_utils::ts_point_to_lsp_position,
};
use zap_language::tree_sitter_utils::AtomIterator;

mod instances;
mod keywords;
mod namespaces;
mod options;
mod properties;
mod types;

pub use self::instances::completion as completion_for_instances;
pub use self::keywords::completion as completion_for_keywords;
pub use self::namespaces::completion as completion_for_namespaces;
pub use self::options::completion as completion_for_options;
pub use self::properties::completion as completion_for_properties;
pub use self::types::completion as completion_for_types;

pub fn completion_trigger_characters() -> Vec<String> {
    let mut chars = vec![
        String::from("\""),
        String::from("'"),
        String::from("/"),
        String::from("@"),
        String::from(":"),
        String::from("."),
        String::from("-"),
        String::from("_"),
        String::from(" "),
    ];

    chars.sort();
    chars
}

/**
    Modifies the given position to be more accurate for completions.

    When the position is over whitespace, this will "stick" the position
    to the left-most nearest named node, making any positions such as
    "field: |" where "|" is the position into "field:| " instead.

    This lets us match on identifiers and other types of nodes even when
    they are completely empty, leading to a much better completion experience.
*/
pub fn completion_pos(doc: &Document, pos: Position) -> Position {
    if let Some(node) = doc.node_at_position_named(pos) {
        let atoms = AtomIterator::new(node).collect::<Vec<Node>>();

        /*
            If we are past the last atom, and:

            a. It was a named node (not punctuation)
            b. It was empty, meaning it is safe to say that all
               following whitespace is actually part of the node

            Then, we will return a position that hangs on the very end of it
        */
        if let Some(end_pos) = atoms
            .last()
            .filter(|n| n.is_named())
            .filter(|n| doc.node_text(**n).trim().is_empty())
            .and_then(|n| {
                let end_point = n.range().end_point;
                let end_pos = ts_point_to_lsp_position(end_point);
                if pos > end_pos { Some(end_pos) } else { None }
            })
        {
            tracing::trace!("(1) Modified completion pos from {pos:?} to {end_pos:?}");
            return end_pos;
        }

        // Find the left + right atoms surrounding the current position for more searching...
        let idx = atoms.iter().position(|n| {
            let end_point = n.range().end_point;
            let end_pos = ts_point_to_lsp_position(end_point);
            pos <= end_pos
        });
        let left = idx.and_then(|i| atoms.get(i.saturating_sub(1))).copied();
        let right = idx.and_then(|i| atoms.get(i)).copied();

        /*
            If we are between two atoms (left + right), and

            a. Left was a named node (not punctuation)
            b. Left was empty, meaning it is safe to say that all
               following whitespace is actually part of the node
            c. Right is *not* a named node (it is punctuation)

            Then, we will return a position that hangs on the very end of the left node
        */
        if let (Some(l), Some(r)) = (left, right) {
            if l.is_named() && !r.is_named() && doc.node_text(l).trim().is_empty() {
                let end_point = l.range().end_point;
                let end_pos = ts_point_to_lsp_position(end_point);
                tracing::trace!("(2) Modified completion pos from {pos:?} to {end_pos:?}");
                return end_pos;
            }
        }
    }

    pos
}
