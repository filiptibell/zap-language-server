pub type Diagnostic = codespan_reporting::diagnostic::Diagnostic<()>;
pub type Severity = codespan_reporting::diagnostic::Severity;
pub type LabelStyle = codespan_reporting::diagnostic::LabelStyle;

#[must_use]
pub fn parse(contents: &str) -> Vec<Diagnostic> {
    libzap::parser::parse(contents)
        .1
        .into_iter()
        .map(|report| report.to_diagnostic(false))
        .collect()
}
