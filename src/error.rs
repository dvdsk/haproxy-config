use std::borrow::Cow;
use std::path::PathBuf;

use ariadne::{Label, Report, ReportKind, Source};

use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Debug)]
pub struct Error<'input> {
    pub inner: ParseError<LineCol>,
    pub source: &'input str,
    pub path: Option<PathBuf>,
}

impl<'i> Error<'i> {
    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn print(&self) {
        let offset = self.inner.location.offset;
        let msg = format!("expected {}", self.inner.expected);

        let path = self
            .path
            .as_ref()
            .map(|p| p.to_string_lossy())
            .map(Cow::into_owned)
            .unwrap_or("<unknown>".to_string());

        Report::build(ReportKind::Error, &path, offset)
            .with_message("parse error".to_string())
            .with_label(Label::new((&path, offset..offset + 1)).with_message(msg))
            .finish()
            .print((&path, Source::from(self.source)))
            .unwrap();
    }

    pub fn eprint(&self) {
        let offset = self.inner.location.offset;
        let msg = format!("expected {}", self.inner.expected);

        let path = self
            .path
            .as_ref()
            .map(|p| p.to_string_lossy())
            .map(Cow::into_owned)
            .unwrap_or("<unknown>".to_string());

        Report::build(ReportKind::Error, &path, offset)
            .with_message("parse error".to_string())
            .with_label(Label::new((&path, offset..offset + 1)).with_message(msg))
            .finish()
            .eprint((&path, Source::from(self.source)))
            .unwrap();
    }
}
