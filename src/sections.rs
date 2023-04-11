use std::net::Ipv4Addr;

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
        lines: Vec<Line<'input>>,
    },
    /// The lines in the default section of a config.
    Default {
        /// Comment on the same line as the section header
        comment: Option<&'input str>,
        /// The default proxy stated after the section header
        proxy: Option<&'input str>,
        /// [Lines](Line) in this section.
        lines: Vec<Line<'input>>,
    },
    Frontend {
        /// Comment on the same line as the section header
        comment: Option<&'input str>,
        /// The proxy stated after the section header
        proxy: &'input str,
        /// [Lines](Line) in this section.
        lines: Vec<Line<'input>>,
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
        lines: Vec<Line<'input>>,
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
        lines: Vec<Line<'input>>,
    },
    Userlist {
        /// Comment on the same line as the section header
        comment: Option<&'input str>,
        /// Name of this userlist
        name: &'input str,
        /// [Lines](Line) in this section.
        lines: Vec<Line<'input>>,
    },
    UnknownLine {
        /// A line that could not be parsed.
        line: &'input str,
    },
}

/// See [Host](super::config::Host) for an owned variant.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HostRef<'input> {
    Ipv4(Ipv4Addr),
    Dns(&'input str),
    Wildcard,
}

/// See [Address](super::config::Address) for an owned variant.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AddressRef<'input> {
    pub host: HostRef<'input>,
    pub port: Option<u16>,
}

/// In combination with an [Acl](Line::Acl) forms the condition for picking a backend
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum BackendModifier {
    If,
    Unless,
}

/// See [Password](super::config::Password) for an owned variant.
#[derive(Debug)]
pub enum PasswordRef<'input> {
    Secure(&'input str),
    Insecure(&'input str),
}

/// Various lines that can occure in config [sections](Section)
#[derive(Debug)]
pub enum Line<'input> {
    Server {
        name: &'input str,
        addr: AddressRef<'input>,
        option: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Option {
        keyword: &'input str,
        value: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Bind {
        addr: AddressRef<'input>,
        value: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Acl {
        name: &'input str,
        rule: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Backend {
        name: &'input str,
        modifier: Option<BackendModifier>,
        condition: Option<&'input str>,
        comment: Option<&'input str>,
    },
    /// This usually refers to a haproxy user group however if it is in the global section of a
    /// config it is the systemuser haproxy will try to run as after starting up.
    Group {
        name: &'input str,
        users: Vec<&'input str>,
        comment: Option<&'input str>,
    },
    User {
        name: &'input str,
        password: PasswordRef<'input>,
        groups: Vec<&'input str>,
        comment: Option<&'input str>,
    },
    /// A global paramater describing the system user haproxy should run as
    SysUser {
        name: &'input str,
    },
    Config {
        key: &'input str,
        value: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Comment(&'input str),
    Blank,
}