use std::net::Ipv4Addr;

#[derive(Debug)]
pub enum ConfigSection<'input> {
    BlankLine,
    Comment(&'input str),
    Global {
        comment: Option<&'input str>,
        lines: Vec<Line<'input>>,
    },
    Default {
        comment: Option<&'input str>,
        proxy: Option<&'input str>,
        lines: Vec<Line<'input>>,
    },
    Frontend {
        comment: Option<&'input str>,
        proxy: &'input str,
        lines: Vec<Line<'input>>,
        /// Optional address to which the frontend binds can be stated
        /// in the header, for example `frontend webserver *:80` instead
        /// of a bind line, any optional config for the bind follows
        header_addr: Option<(AddressRef<'input>, Option<&'input str>)>,
    },
    Listen {
        comment: Option<&'input str>,
        proxy: &'input str,
        lines: Vec<Line<'input>>,
        /// Optional address to which the listen binds can be stated
        /// in the header, for example `frontend webserver *:80` instead
        /// of a bind line, any optional config for the bind follows
        header_addr: Option<(AddressRef<'input>, Option<&'input str>)>,
    },
    Backend {
        comment: Option<&'input str>,
        proxy: &'input str,
        lines: Vec<Line<'input>>,
    },
    Userlist {
        comment: Option<&'input str>,
        name: &'input str,
        lines: Vec<Line<'input>>,
    },
    UnknownLine {
        line: &'input str,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum HostRef<'input> {
    Ipv4(Ipv4Addr),
    Dns(&'input str),
    Wildcard,
}

#[derive(Debug)]
pub enum Host {
    Ipv4(Ipv4Addr),
    Dns(String),
    Wildcard,
}

impl From<&HostRef<'_>> for Host {
    fn from(h: &HostRef<'_>) -> Self {
        match h {
            HostRef::Ipv4(a) => Host::Ipv4(*a),
            HostRef::Dns(s) => Host::Dns(s.to_string()),
            HostRef::Wildcard => Host::Wildcard,
        }
    }
}

impl From<&AddressRef<'_>> for Address {
    fn from(r: &AddressRef<'_>) -> Self {
        Address {
            host: Host::from(&r.host),
            port: r.port,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AddressRef<'input> {
    pub host: HostRef<'input>,
    pub port: Option<u16>,
}

#[derive(Debug)]
pub struct Address {
    pub host: Host,
    pub port: Option<u16>,
}

#[derive(Debug, Clone)]
pub enum BackendModifier {
    If,
    Unless,
}

#[derive(Debug)]
pub enum PasswordRef<'input> {
    Secure(&'input str),
    Insecure(&'input str),
}

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
