use crate::config::Address;
use crate::line::owned;

/// Represents a section in a config file.
#[derive(Debug)]
pub enum Section {
    BlankLine,
    /// Comment on a separate line not in a section
    Comment(String),
    /// The global section of a config.
    Global {
        /// Comment on the same line as the section header
        comment: Option<String>,
        /// [`Lines`](owned::Line) in this section.
        lines: Vec<owned::Line>,
    },
    /// The lines in the default section of a config.
    Default {
        /// Comment on the same line as the section header
        comment: Option<String>,
        /// The default proxy stated after the section header
        proxy: Option<String>,
        /// [`Lines`](owned::Line) in this section.
        lines: Vec<owned::Line>,
    },
    Frontend {
        /// Comment on the same line as the section header
        comment: Option<String>,
        /// The proxy stated after the section header
        proxy: String,
        /// [`Lines`](owned::Line) in this section.
        lines: Vec<owned::Line>,
        /// Optional address to which the frontend binds can be stated
        /// in the header, for example `frontend webserver *:80` instead
        /// of a bind line, any optional config for the bind follows
        header_addr: Option<(Address, Option<String>)>,
    },
    Listen {
        /// Comment on the same line as the section header
        comment: Option<String>,
        /// The proxy stated after the section header
        proxy: String,
        /// [`Lines`](owned::Line) in this section.
        lines: Vec<owned::Line>,
        /// Optional address to which the listen binds can be stated
        /// in the header, for example `frontend webserver *:80` instead
        /// of a bind line, any optional config for the bind follows
        header_addr: Option<(Address, Option<String>)>,
    },
    Backend {
        /// Comment on the same line as the section header
        comment: Option<String>,
        /// The proxy stated after the section header
        proxy: String,
        /// [`Lines`](owned::Line) in this section.
        lines: Vec<owned::Line>,
    },
    Userlist {
        /// Comment on the same line as the section header
        comment: Option<String>,
        /// Name of this userlist
        name: String,
        /// [`Lines`](owned::Line) in this section.
        lines: Vec<owned::Line>,
    },
    UnknownLine {
        /// A line that could not be parsed.
        line: String,
    },
}
