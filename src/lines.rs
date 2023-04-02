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
    },
    Backend {
        comment: Option<&'input str>,
        proxy: &'input str,
        lines: Vec<Line<'input>>,
    },
    UserList {
        comment: Option<&'input str>,
        proxy: &'input str,
        lines: Vec<Line<'input>>,
    },
    Listen {
        comment: Option<&'input str>,
        proxy: &'input str,
        lines: Vec<Line<'input>>,
    },
}

#[derive(Debug)]
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

#[derive(Debug)]
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
pub enum Password<'input> {
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
        user: Option<&'input str>,
        comment: Option<&'input str>,
    },
    User {
        name: &'input str,
        password: Password<'input>,
        groups: Vec<&'input str>,
        comment: Option<&'input str>, 
    },
    Config {
        key: &'input str,
        value: Option<&'input str>,
        comment: Option<&'input str>, 
    },
    Comment(&'input str),
    Blank,
}


