// http://docs.haproxy.org/2.7/configuration.html#4.1

use itertools::{Itertools};
use std::collections::HashMap;

use crate::sections::{Line, Address};
use super::sections::ConfigSection;

mod global;
pub use global::Global;
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
    pub name: String,
    pub rule: String,
}

#[derive(Debug)]
pub struct Bind {
    pub addr: Address,
    pub config: Option<String>,
}

#[derive(Debug)]
pub struct Server {
    pub name: String,
    pub addr: Address,
    pub option: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub global: Global,
    pub default: Default,
    pub frontends: Vec<Frontend>,
    pub backends: Vec<Backend>,
    pub listen: Vec<Listen>,
    pub userlists: Vec<Userlist>,
}

impl<'a> TryFrom<&'a [ConfigSection<'a>]> for Config {
    type Error = Error<'a>;

    fn try_from(entries: &'a [ConfigSection<'a>]) -> Result<Self, Self::Error> {
        let unknown_lines = entries.iter().filter_map(|l| match l {
            ConfigSection::UnknownLine { line } => Some(*line),
            _ => None,
        }).collect_vec();
        
        if !unknown_lines.is_empty() {
            return Err(Error::UnknownLines(unknown_lines));
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

#[derive(Debug)]
pub enum Error<'a> {
    UnknownLines(Vec<&'a str>),
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
    MultipleSysUsers(Vec<&'a Line<'a>>),
    MultipleSysGroups(Vec<&'a Line<'a>>),
    SysUserHasGroups(&'a Line<'a>),
    SysGroupHasUsers(&'a Line<'a>),
}


#[derive(Debug, Default)]
pub struct Default {
    pub proxy: Option<String>,
    pub config: HashMap<String, Option<String>>,
    pub options: HashMap<String, Option<String>>,
}

impl<'a> TryFrom<&'a [ConfigSection<'a>]> for Default {
    type Error = Error<'a>;

    fn try_from(entries: &'a [ConfigSection<'_>]) -> Result<Self, Self::Error> {
        let default_entries: Vec<_> = entries
            .iter()
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
