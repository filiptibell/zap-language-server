use std::collections::VecDeque;

use async_lsp::lsp_types::{Position as LspPosition, Range as LspRange};
use tree_sitter::{Node, Point as TsPoint, Range as TsRange};

/**
    Converts a tree sitter `Point` to an LSP `Position`
*/
#[must_use]
pub const fn ts_point_to_lsp_position(pos: TsPoint) -> LspPosition {
    LspPosition {
        line: pos.row as u32,
        character: pos.column as u32,
    }
}

/**
    Converts a tree sitter `Range` to an LSP `Range`
*/
#[must_use]
pub const fn ts_range_to_lsp_range(range: TsRange) -> LspRange {
    LspRange {
        start: ts_point_to_lsp_position(range.start_point),
        end: ts_point_to_lsp_position(range.end_point),
    }
}

/**
    Returns `true` if the given tree sitter `Range`
    contains the given LSP `Position`, otherwise `false`

    This is an **inclusive** bounds check, meaning the position is
    considered *inside* even if it lies on a line or column boundary
*/
#[must_use]
pub const fn ts_range_contains_lsp_position(range: TsRange, pos: LspPosition) -> bool {
    let point = lsp_position_to_ts_point(pos);
    point.row >= range.start_point.row
        && point.column >= range.start_point.column
        && point.row <= range.end_point.row
        && point.column <= range.end_point.column
}

/**
    Returns `true` if the given tree sitter `Range`
    contains the given tree sitter `Point`, otherwise `false`

    This is an **inclusive** bounds check, meaning the point is
    considered *inside* even if it lies on a line or column boundary
*/
#[must_use]
pub const fn ts_range_contains_ts_point(range: TsRange, point: TsPoint) -> bool {
    point.row >= range.start_point.row
        && point.column >= range.start_point.column
        && point.row <= range.end_point.row
        && point.column <= range.end_point.column
}

/**
    Converts an LSP `Position` to a tree sitter `Point`
*/
#[must_use]
pub const fn lsp_position_to_ts_point(pos: LspPosition) -> TsPoint {
    TsPoint {
        row: pos.line as usize,
        column: pos.character as usize,
    }
}

/**
    Finds the first child node that matches the given predicate.
*/
#[must_use]
pub fn find_child<'a, F>(node: Node<'a>, predicate: F) -> Option<Node<'a>>
where
    F: Fn(&Node<'a>) -> bool,
{
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if predicate(&child) {
            return Some(child);
        }
    }
    None
}

/**
    Finds the first ancestor node that matches the given predicate.
*/
#[must_use]
pub fn find_ancestor<'a, F>(node: Node<'a>, predicate: F) -> Option<Node<'a>>
where
    F: Fn(&Node<'a>) -> bool,
{
    let mut current = node.parent();

    while let Some(node) = current {
        if predicate(&node) {
            return Some(node);
        }
        current = node.parent();
    }

    None
}

/**
    Finds the first descendant node that matches the given predicate.

    This will search descendants in a depth-first manner.
*/
#[must_use]
pub fn find_descendant<'a, F>(node: Node<'a>, predicate: F) -> Option<Node<'a>>
where
    F: Fn(&Node<'a>) -> bool,
{
    let mut cursor = node.walk();
    let mut stack = VecDeque::from([node]);

    while let Some(current) = stack.pop_front() {
        if predicate(&current) {
            return Some(current);
        }
        for child in current.children(&mut cursor) {
            stack.push_back(child);
        }
    }

    None
}

/**
    Finds the descendant node that directly contains the given point.

    This uses **inclusive** bounds checks, meaning that points are
    considered *inside* even if they lie on a line or column boundary
*/
pub fn find_descendant_at_point<'a>(node: Node<'a>, point: TsPoint) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    let mut current = None;

    if ts_range_contains_ts_point(node.range(), point.clone()) {
        current = Some(node);
    }

    'outer: while let Some(node) = current.take() {
        for child in node.children(&mut cursor) {
            if ts_range_contains_ts_point(child.range(), point.clone()) {
                current = Some(child);
                continue 'outer;
            }
        }
        return Some(node);
    }

    None
}
