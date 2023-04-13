use std::borrow::Cow;
use std::io::BufWriter;
use std::path::PathBuf;

use ariadne::{Label, Report, ReportKind, Source};

use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Debug)]
pub struct Error {
    pub inner: ParseError<LineCol>,
    pub source: String,
    pub path: Option<PathBuf>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bytes = Vec::new();
        {
            let buf = BufWriter::new(&mut bytes);

            self.report()
                .write((self.path(), Source::from(&self.source)), buf)
                .unwrap();
        }

        let string = String::from_utf8(bytes).expect("source is utf8 thus report is utf8");
        f.write_str(&string)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl Error {
    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn report(&self) -> Report<(String, std::ops::Range<usize>)> {
        let offset = self.inner.location.offset;
        let msg = format!("expected {}", self.inner.expected);

        Report::build(ReportKind::Error, self.path(), offset)
            .with_message("parse error".to_string())
            .with_label(Label::new((self.path(), offset..offset)).with_message(msg))
            .finish()
    }

    fn path(&self) -> String {
        let path = self
            .path
            .as_ref()
            .map(|p| (*p).to_string_lossy())
            .map_or_else(|| "<unknown>".to_string(), Cow::into_owned);
        path
    }
}
