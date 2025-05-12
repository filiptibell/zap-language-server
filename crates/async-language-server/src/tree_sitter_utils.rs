use async_lsp::lsp_types::{Position as LspPosition, Range as LspRange};
use tree_sitter::{Point as TsPoint, Range as TsRange};

pub const fn ts_point_to_lsp_position(pos: TsPoint) -> LspPosition {
    LspPosition {
        line: pos.row as u32,
        character: pos.column as u32,
    }
}

pub const fn ts_range_to_lsp_range(range: TsRange) -> LspRange {
    LspRange {
        start: ts_point_to_lsp_position(range.start_point),
        end: ts_point_to_lsp_position(range.end_point),
    }
}

pub fn ts_range_contains_lsp_position(range: TsRange, pos: LspPosition) -> bool {
    let point = lsp_position_to_ts_point(pos);
    range.start_point <= point && point <= range.end_point
}

pub const fn lsp_position_to_ts_point(pos: LspPosition) -> TsPoint {
    TsPoint {
        row: pos.line as usize,
        column: pos.character as usize,
    }
}
