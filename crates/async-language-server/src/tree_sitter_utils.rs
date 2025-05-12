use async_lsp::lsp_types::{Position as LspPosition, Range as LspRange};
use tree_sitter::{Point as TsPoint, Range as TsRange};

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
    Converts an LSP `Position` to a tree sitter `Point`
*/
#[must_use]
pub const fn lsp_position_to_ts_point(pos: LspPosition) -> TsPoint {
    TsPoint {
        row: pos.line as usize,
        column: pos.character as usize,
    }
}
