// http://docs.haproxy.org/2.7/configuration.html#4.1

use itertools::{Either, Itertools};
use std::collections::HashMap;

use crate::lines::{Line, Address};
use super::lines::ConfigSection;

mod frontend;
pub use frontend::Frontend;
mod backend;
pub use backend::Backend;
mod listen;
pub use listen::Listen;
mod userlist;
pub use userlist::Userlist;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Acl {
    name: String,
    rule: String,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Bind {
    addr: Address,
    config: Option<String>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Server {
    name: String,
    addr: Address,
    option: Option<String>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Config {
    global: Global,
    default: Default,
    frontends: Vec<Frontend>,
    backends: Vec<Backend>,
    listen: Vec<Listen>,
    userlists: Vec<Userlist>,
}

impl<'a> TryFrom<&'a [ConfigSection<'a>]> for Config {
    type Error = Error<'a>;

    fn try_from(entries: &'a [ConfigSection<'a>]) -> Result<Self, Self::Error> {
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

#[derive(Debug)]
pub enum Error<'a> {
    MissingGlobal,
    MultipleGlobalEntries(Vec<&'a ConfigSection<'a>>),
    WrongGlobalLines(Vec<&'a Line<'a>>),
    MultipleDefaultEntries(Vec<&'a ConfigSection<'a>>),
    AclWithoutRule(&'a str),
    WrongFrontendLines(Vec<&'a Line<'a>>),
    MoreThenOneBind(Vec<&'a Line<'a>>),
    NoBind,
    HeaderAndBindLine,
    WrongListenLines(Vec<&'a Line<'a>>),
    WrongBackendLines(Vec<&'a Line<'a>>),
    WrongUserlistLines(Vec<&'a Line<'a>>),
    WrongDefaultLines(Vec<&'a Line<'a>>),
}

#[derive(Debug, Default)]
pub struct Global {
    pub config: HashMap<String, Option<String>>,
}

impl<'a> TryFrom<&'a [ConfigSection<'a>]> for Global {
    type Error = Error<'a>;

    fn try_from(entries: &'a [ConfigSection<'a>]) -> Result<Self, Self::Error> {
        let global_entries: Vec<_> = entries
            .into_iter()
            .filter(|e| matches!(e, ConfigSection::Global { .. }))
            .collect();

        if global_entries.len() > 1 {
            return Err(Error::MultipleGlobalEntries(global_entries));
        }

        let Some(ConfigSection::Global{ lines, ..}) = global_entries.first() else {
            return Ok(Global::default());
        };

        let (config, other) = extract_config(lines);

        if !other.is_empty() {
            return Err(Error::WrongGlobalLines(other));
        }

        Ok(Global { config })
    }
}

fn extract_config<'a>(
    lines: &'a Vec<Line<'a>>,
) -> (HashMap<String, Option<String>>, Vec<&'a Line<'a>>) {
    let (config, other): (HashMap<_, Option<_>>, Vec<_>) = lines
        .iter()
        .filter(|l| !matches!(l, Line::Blank | Line::Comment(_)))
        .inspect(|l| eprintln!("mmm ****, {:?}", l))
        .partition_map(|l| match l {
            Line::Config { key, value, .. } => {
                let key = key.to_string();
                let value = value.map(ToOwned::to_owned);
                Either::Left((key, value))
            }
            _other => Either::Right(_other),
        });
    (config, other)
}

#[allow(unused)]
#[derive(Debug, Default)]
pub struct Default {
    proxy: Option<String>,
    config: HashMap<String, Option<String>>,
    options: HashMap<String, Option<String>>,
}

impl<'a> TryFrom<&'a [ConfigSection<'a>]> for Default {
    type Error = Error<'a>;

    fn try_from(entries: &'a [ConfigSection<'_>]) -> Result<Self, Self::Error> {
        let default_entries: Vec<_> = entries
            .into_iter()
            .filter(|e| matches!(e, ConfigSection::Default { .. }))
            .collect();

        if default_entries.len() > 1 {
            return Err(Error::MultipleDefaultEntries(default_entries));
        }

        let Some(ConfigSection::Default{ proxy, lines, ..}) = default_entries.first() else {
            return Ok(Default::default());
        };

        let mut config = HashMap::new();
        let mut options = HashMap::new();
        let mut other = Vec::new();
        for line in lines.iter().filter(|l| !matches!(l, Line::Blank | Line::Comment(_))) {
            match line {
                Line::Config { key, value, .. } => {
                    let key = key.to_string();
                    let value = value.map(ToOwned::to_owned);
                    config.insert(key, value);
                }
                Line::Option {
                    keyword: key,
                    value,
                    ..
                } => {
                    let key = key.to_string();
                    let value = value.map(ToOwned::to_owned);
                    options.insert(key, value);
                }
                _other => other.push(_other),
            }
        }

        if !other.is_empty() {
            return Err(Error::WrongDefaultLines(other));
        }

        Ok(Default {
            proxy: proxy.map(ToOwned::to_owned),
            config,
            options,
        })
    }
}
