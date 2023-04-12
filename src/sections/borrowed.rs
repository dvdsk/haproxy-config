use super::AddressRef;

use super::lines::borrowed;

/// Parsed haproxy config preserving the order and comments.
/// Does not support conditional blocks,
/// these and other unsupported lines will be stored in the [UnknownLine](Section::UnknownLine) variant.
/// Information outside the header is containd in the correct order in the lines member. See the [Line] documentation.
#[derive(Debug)]
pub enum Section<'input> {
    BlankLine,
    /// Comment on a seperate line not in a section
    Comment(&'input str),
    /// The global section of a config.
    Global {
        /// Comment on the same line as the section header
        comment: Option<&'input str>,
        /// [Lines](Line) in this section.
        lines: Vec<borrowed::Line<'input>>,
    },
    /// The lines in the default section of a config.
    Default {
        /// Comment on the same line as the section header
        comment: Option<&'input str>,
        /// The default proxy stated after the section header
        proxy: Option<&'input str>,
        /// [Lines](Line) in this section.
        lines: Vec<borrowed::Line<'input>>,
    },
    Frontend {
        /// Comment on the same line as the section header
        comment: Option<&'input str>,
        /// The proxy stated after the section header
        proxy: &'input str,
        /// [Lines](Line) in this section.
        lines: Vec<borrowed::Line<'input>>,
        /// Optional address to which the frontend binds can be stated
        /// in the header, for example `frontend webserver *:80` instead
        /// of a bind line, any optional config for the bind follows
        header_addr: Option<(AddressRef<'input>, Option<&'input str>)>,
    },
    Listen {
        /// Comment on the same line as the section header
        comment: Option<&'input str>,
        /// The proxy stated after the section header
        proxy: &'input str,
        /// [Lines](Line) in this section.
        lines: Vec<borrowed::Line<'input>>,
        /// Optional address to which the listen binds can be stated
        /// in the header, for example `frontend webserver *:80` instead
        /// of a bind line, any optional config for the bind follows
        header_addr: Option<(AddressRef<'input>, Option<&'input str>)>,
    },
    Backend {
        /// Comment on the same line as the section header
        comment: Option<&'input str>,
        /// The proxy stated after the section header
        proxy: &'input str,
        /// [Lines](Line) in this section.
        lines: Vec<borrowed::Line<'input>>,
    },
    Userlist {
        /// Comment on the same line as the section header
        comment: Option<&'input str>,
        /// Name of this userlist
        name: &'input str,
        /// [Lines](Line) in this section.
        lines: Vec<borrowed::Line<'input>>,
    },
    UnknownLine {
        /// A line that could not be parsed.
        line: &'input str,
    },
}
