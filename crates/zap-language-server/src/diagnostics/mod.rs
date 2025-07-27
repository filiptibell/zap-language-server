use std::ops::Range;

use async_language_server::{
    lsp_types::{
        Diagnostic as LspDiagnostic,
        DiagnosticRelatedInformation as LspDiagnosticRelatedInformation,
        DiagnosticSeverity as LspDiagnosticSeverity, DiagnosticTag as LspDiagnosticTag,
        Location as LspLocation, NumberOrString, Position as LspPosition, Range as LspRange,
    },
    server::Document,
};

use zap_language::diagnostics::{Diagnostic, LabelStyle, Severity};

pub fn zap_diagnostic_to_lsp_diagnostic(
    document: &Document,
    diagnostic: Diagnostic,
) -> Option<LspDiagnostic> {
    let primary_label = diagnostic
        .labels
        .iter()
        .find(|label| label.style == LabelStyle::Primary)?;

    let range = byte_range_to_lsp_range(document, primary_label.range.clone())?;
    let severity = match diagnostic.severity {
        Severity::Help => Some(LspDiagnosticSeverity::HINT),
        Severity::Bug | Severity::Note => Some(LspDiagnosticSeverity::INFORMATION),
        Severity::Warning => Some(LspDiagnosticSeverity::WARNING),
        Severity::Error => Some(LspDiagnosticSeverity::ERROR),
    };

    let mut message = String::new();
    message.extend(sentence_chars(&diagnostic.message));
    if !diagnostic.notes.is_empty() {
        message.push('\n');
        for note in diagnostic.notes {
            message.push_str("\n- ");
            message.extend(sentence_chars(&note));
            message.push('.');
        }
        message.push('\n');
    }

    let mut tags = Vec::new();
    if let Some(code) = diagnostic.code.as_ref().and_then(|c| c.parse::<u32>().ok()) {
        if (4000..5000).contains(&code) {
            tags.push(LspDiagnosticTag::DEPRECATED);
        }
    }

    let related_information = diagnostic
        .labels
        .iter()
        .filter(|label| *label != primary_label)
        .filter_map(|label| {
            Some(LspDiagnosticRelatedInformation {
                message: label.message.clone(),
                location: LspLocation {
                    uri: document.url().clone(),
                    range: byte_range_to_lsp_range(document, label.range.clone())?,
                },
            })
        })
        .collect::<Vec<_>>();

    Some(LspDiagnostic {
        range,
        severity,
        message,
        tags: Some(tags),
        source: Some(String::from("Zap")),
        code: diagnostic.code.map(NumberOrString::String),
        related_information: Some(related_information),
        ..Default::default()
    })
}

// codespan-reporting uses byte ranges, but we need line + char positions

fn byte_range_to_lsp_range(document: &Document, byte_range: Range<usize>) -> Option<LspRange> {
    Some(LspRange {
        start: byte_offset_to_position(document, byte_range.start)?,
        end: byte_offset_to_position(document, byte_range.end)?,
    })
}

#[allow(clippy::cast_possible_truncation)]
fn byte_offset_to_position(document: &Document, byte_offset: usize) -> Option<LspPosition> {
    let text = document.text();

    let line_index = text.try_byte_to_line(byte_offset).ok()?;
    let line_byte = text.try_line_to_byte(line_index).ok()?;

    Some(LspPosition {
        line: line_index as u32,
        character: (byte_offset - line_byte) as u32,
    })
}

// zap diagnostics are always all lowercase, we use this to make first chars uppercase

fn sentence_chars(s: &str) -> impl Iterator<Item = char> {
    let mut first = true;
    s.chars().map(move |c| {
        if first {
            first = false;
            c.to_ascii_uppercase()
        } else {
            c
        }
    })
}
