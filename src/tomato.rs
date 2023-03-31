use std::net::Ipv4Addr;

#[derive(Debug)]
pub enum ConfigEntry<'input> {
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
pub enum Host<'input> {
    Ipv4(Ipv4Addr),
    Dns(&'input str),
    Wildcard,
}

#[derive(Debug)]
pub struct Address<'input> {
    pub host: Host<'input>,
    pub port: Option<u16>,
}

#[derive(Debug)]
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
        addr: Address<'input>,
        other: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Option {
        keyword: &'input str,
        value: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Bind {
        addr: Address<'input>,
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


