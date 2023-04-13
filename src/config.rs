// http://docs.haproxy.org/2.7/configuration.html#4.1

use std::collections::HashMap;
use std::net::Ipv4Addr;

use super::section::borrowed::Section;
use crate::section::{AddressRef, HostRef};
use crate::line::borrowed::Line; 

mod global;
pub use global::Global;
mod frontend;
pub use frontend::Frontend;
mod backend;
pub use backend::Backend;
mod listen;
pub use listen::Listen;
mod userlist;
pub use userlist::{Userlist, User, Group, Password};
mod error;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Acl {
    pub name: String,
    pub rule: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Bind {
    pub addr: Address,
    pub config: Option<String>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Server {
    pub name: String,
    pub addr: Address,
    pub option: Option<String>,
}

/// Owned variant of [`AddressRef`]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Address {
    pub host: Host,
    pub port: Option<u16>,
}

impl From<&AddressRef<'_>> for Address {
    fn from(r: &AddressRef<'_>) -> Self {
        Address {
            host: Host::from(&r.host),
            port: r.port,
        }
    }
}

/// Owned variant of [`HostRef`]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Host {
    Ipv4(Ipv4Addr),
    Dns(String),
    Wildcard,
}

impl From<&HostRef<'_>> for Host {
    fn from(h: &HostRef<'_>) -> Self {
        match h {
            HostRef::Ipv4(a) => Host::Ipv4(*a),
            HostRef::Dns(s) => Host::Dns((*s).to_string()),
            HostRef::Wildcard => Host::Wildcard,
        }
    }
}

pub type Name = String;

/// A haproxy config where everything except config and options are fully typed. Can be created
/// from a list of [`Sections`](Section) using [`TryFrom`]. This type does not borrow its input.
///
/// Returns Err if the config contains errors or sections or grammar we don not supported. For
/// example conditional blocks.
///
/// # Examples
/// Build a config from a list of just parsed sections.
/// ```
/// use haproxy_config::parse_sections;
/// use haproxy_config::Config;
///
/// let file = include_str!("../tests/medium_haproxy.cfg");
/// let sections = parse_sections(file).unwrap();
///
/// let config = Config::try_from(&sections).unwrap();
/// ```
///
/// The same as above however we filter out unknown lines that
/// would result in an error. This only works for lines
/// above the first section as anything unknown after a section starts
/// is parsed as a config option.
/// ```
/// use haproxy_config::parse_sections;
/// use haproxy_config::{Config, section::borrowed::Section};
///
/// let file = include_str!("../tests/unsupported/nonesens.cfg");
/// let sections = dbg!(parse_sections(file).unwrap());
/// let supported_sections: Vec<_> = sections.into_iter().filter(|s| !std::matches!(s,
/// Section::UnknownLine{..})).collect();
///
/// let config = Config::try_from(&supported_sections).unwrap();
/// 
/// // `nonesens.cfg` contained the line `this will be seen as config unfortunatly`
/// // its stats frontend. This is not recognised as an error unfortunatly:
/// assert_eq!(config.frontends
///     .get("stats")
///     .unwrap()
///     .config.get("this")
///     .unwrap()
///     .as_ref()
///     .unwrap(), "will be seen as a config value unfortunatly");
/// ```
#[derive(Debug)]
pub struct Config {
    pub global: Global,
    pub default: Default,
    pub frontends: HashMap<Name, Frontend>,
    pub backends: HashMap<Name, Backend>,
    pub listen: HashMap<Name, Listen>,
    pub userlists: HashMap<Name, Userlist>,
}

impl<'a> TryFrom<&'a Vec<Section<'a>>> for Config {
    type Error = error::Error;

    fn try_from(vec: &'a Vec<Section<'a>>) -> Result<Self, Self::Error> {
        Config::try_from(vec.as_slice())
    }
}

impl<'a> TryFrom<&'a [Section<'a>]> for Config {
    type Error = error::Error;

    fn try_from(entries: &'a [Section<'a>]) -> Result<Self, Self::Error> {
        let unknown_lines: Vec<_> = entries
            .iter()
            .filter_map(|l| match l {
                Section::UnknownLine { line } => Some(*line),
                _ => None,
            })
            .collect();

        if !unknown_lines.is_empty() {
            return Err(error::Error::unknown_lines(unknown_lines));
        }

        Ok(Config {
            global: Global::try_from(entries)?,
            default: Default::try_from(entries)?,
            frontends: Frontend::parse_multiple(entries)?,
            backends: Backend::parse_multiple(entries)?,
            listen: Listen::parse_multiple(entries)?,
            userlists: Userlist::parse_multiple(entries)?,
        })
    }
}


#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Default {
    pub proxy: Option<String>,
    pub config: HashMap<String, Option<String>>,
    pub options: HashMap<String, Option<String>>,
}

impl<'a> TryFrom<&'a [Section<'a>]> for Default {
    type Error = error::Error;

    fn try_from(entries: &'a [Section<'_>]) -> Result<Self, Self::Error> {
        let default_entries: Vec<_> = entries
            .iter()
            .filter(|e| matches!(e, Section::Default { .. }))
            .collect();

        if default_entries.len() > 1 {
            return Err(error::Error::multiple_default_entries(default_entries));
        }

        let Some(Section::Default{ proxy, lines, ..}) = default_entries.first() else {
            return Ok(Default::default());
        };

        let mut config = HashMap::new();
        let mut options = HashMap::new();
        let mut other = Vec::new();
        for line in lines
            .iter()
            .filter(|l| !matches!(l, Line::Blank | Line::Comment(_)))
        {
            match line {
                Line::Config { key, value, .. } => {
                    let key = (*key).to_string();
                    let value = value.map(ToOwned::to_owned);
                    config.insert(key, value);
                }
                Line::Option {
                    keyword: key,
                    value,
                    ..
                } => {
                    let key = (*key).to_string();
                    let value = value.map(ToOwned::to_owned);
                    options.insert(key, value);
                }
                o => other.push(o),
            }
        }

        if !other.is_empty() {
            return Err(error::Error::wrong_default_lines(other));
        }

        Ok(Default {
            proxy: proxy.map(ToOwned::to_owned),
            config,
            options,
        })
    }
}
