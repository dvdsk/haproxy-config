use std::net::Ipv4Addr;

/// Zero copy representations of the lines in a config section, fast to create however hard to
/// pass around as it has a lifetime. This is the default returned by
/// [`parse_sections`][crate::parse_sections].
pub mod borrowed;
/// Owned representations of a config section, far easier to work with. Can be created
/// from a [`borrowed::Section`] using [`From`].
pub mod owned;

use crate::line::owned::Line;

use crate::config::Address;

impl<'a> From<&'a borrowed::Section<'a>> for owned::Section {
    fn from(section: &'a borrowed::Section<'a>) -> Self {
        match section {
            borrowed::Section::BlankLine => owned::Section::BlankLine,
            borrowed::Section::Comment(comment) => owned::Section::Comment((*comment).to_string()),
            borrowed::Section::Global { comment, lines } => owned::Section::Global {
                comment: comment.map(ToOwned::to_owned),
                lines: lines.iter().map(Line::from).collect(),
            },
            borrowed::Section::Default {
                comment,
                proxy,
                lines,
            } => owned::Section::Default {
                comment: comment.map(ToOwned::to_owned),
                proxy: proxy.map(ToOwned::to_owned),
                lines: lines.iter().map(Line::from).collect(),
            },
            borrowed::Section::Frontend {
                comment,
                proxy,
                lines,
                header_addr,
            } => owned::Section::Frontend {
                comment: comment.map(ToOwned::to_owned),
                proxy: (*proxy).to_string(),
                lines: lines.iter().map(Line::from).collect(),
                header_addr: header_addr.as_ref().map(|(address_ref, config)| {
                    (Address::from(address_ref), config.map(ToOwned::to_owned))
                }),
            },
            borrowed::Section::Listen {
                comment,
                proxy,
                lines,
                header_addr,
            } => owned::Section::Listen {
                comment: comment.map(ToOwned::to_owned),
                proxy: (*proxy).to_string(),
                lines: lines.iter().map(Line::from).collect(),
                header_addr: header_addr.as_ref().map(|(address_ref, config)| {
                    (Address::from(address_ref), config.map(ToOwned::to_owned))
                }),
            },
            borrowed::Section::Backend {
                comment,
                proxy,
                lines,
            } => owned::Section::Backend {
                comment: comment.map(ToOwned::to_owned),
                proxy: (*proxy).to_string(),
                lines: lines.iter().map(Line::from).collect(),
            },
            borrowed::Section::Userlist {
                comment,
                name,
                lines,
            } => owned::Section::Userlist {
                comment: comment.map(ToOwned::to_owned),
                name: (*name).to_string(),
                lines: lines.iter().map(Line::from).collect(),
            },
            borrowed::Section::UnknownLine { line } => owned::Section::UnknownLine {
                line: (*line).to_string(),
            },
        }
    }
}

/// See [`Host`](super::config::Host) for an owned variant.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HostRef<'input> {
    Ipv4(Ipv4Addr),
    Dns(&'input str),
    Wildcard,
}

/// See [`Address`](super::config::Address) for an owned variant.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AddressRef<'input> {
    pub host: HostRef<'input>,
    pub port: Option<u16>,
}

/// In combination with an [`Acl`](Line::Acl) forms the condition for picking a backend
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum BackendModifier {
    If,
    Unless,
}

/// See [`Password`](super::config::Password) for an owned variant.
#[derive(Debug)]
pub enum PasswordRef<'input> {
    Secure(&'input str),
    Insecure(&'input str),
}
