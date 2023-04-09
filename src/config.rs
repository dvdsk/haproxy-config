// http://docs.haproxy.org/2.7/configuration.html#4.1

use std::collections::HashMap;

use super::sections::Section;
use crate::sections::{Address, Line};

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

impl<'a> TryFrom<&'a [Section<'a>]> for Config {
    type Error = Error<'a>;

    fn try_from(entries: &'a [Section<'a>]) -> Result<Self, Self::Error> {
        let unknown_lines: Vec<_> = entries
            .iter()
            .filter_map(|l| match l {
                Section::UnknownLine { line } => Some(*line),
                _ => None,
            })
            .collect();

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
#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
    #[error("Unknown lines in the input, you should filter these out if you want to ignore them")]
    UnknownLines(Vec<&'a str>),

    #[error("Each config must have a global section")]
    MissingGlobal,

    #[error("The following lines are not allowed in a global section: {0:#?}")]
    WrongGlobalLines(Vec<&'a Line<'a>>),
    #[error("The following lines are not allowed in a listen section: {0:#?}")]
    WrongListenLines(Vec<&'a Line<'a>>),
    #[error("The following lines are not allowed in a backend section: {0:#?}")]
    WrongBackendLines(Vec<&'a Line<'a>>),
    #[error("The following lines are not allowed in a userlist section: {0:#?}")]
    WrongUserlistLines(Vec<&'a Line<'a>>),
    #[error("The following lines are not allowed in a default section: {0:#?}")]
    WrongDefaultLines(Vec<&'a Line<'a>>),
    #[error("The following lines are not allowed in a frontend section: {0:#?}")]
    WrongFrontendLines(Vec<&'a Line<'a>>),

    #[error("There can only be a single global section")]
    MultipleGlobalEntries(Vec<&'a Section<'a>>),
    #[error("There can only be a single default section")]
    MultipleDefaultEntries(Vec<&'a Section<'a>>),

    #[error("Every ACL should have a rule, this one had none: {0}")]
    AclWithoutRule(&'a str),

    #[error("Every frontend or listen section must bind to as address")]
    NoBind,
    #[error("A frontend or listen section may only bind a single address this section has multiple lines binding to an adress: {0:#?}")]
    MoreThenOneBind(Vec<&'a Line<'a>>),
    #[error("A frontend or listen section may only bind a single address however both the header and a bind line further downwere found")]
    HeaderAndBindLine,

    #[error("Processes can only run as single user however multiple where specified: {0:#?}")]
    MultipleSysUsers(Vec<&'a Line<'a>>),
    #[error("Processes can only run as single group however multiple where specified: {0:#?}")]
    MultipleSysGroups(Vec<&'a Line<'a>>),
    #[error("The user line in a global section refers to the user haproxy runs as. It is not the same as an hapoxy user and has no group argument. Add a line \"group <some_unix_group>\" if you want to specify the unix group haproxy should run as.")]
    SysUserHasGroups(&'a Line<'a>),
    #[error("The group line in a global section refers to the user haproxy runs as. It is not the same as an hapoxy user and has no group argument. Add a line \"user <some_unix_user>\" if you want to specify the unix user haproxy should run as.")]
    SysGroupHasUsers(&'a Line<'a>),
}

#[derive(Debug, Default)]
pub struct Default {
    pub proxy: Option<String>,
    pub config: HashMap<String, Option<String>>,
    pub options: HashMap<String, Option<String>>,
}

impl<'a> TryFrom<&'a [Section<'a>]> for Default {
    type Error = Error<'a>;

    fn try_from(entries: &'a [Section<'_>]) -> Result<Self, Self::Error> {
        let default_entries: Vec<_> = entries
            .iter()
            .filter(|e| matches!(e, Section::Default { .. }))
            .collect();

        if default_entries.len() > 1 {
            return Err(Error::MultipleDefaultEntries(default_entries));
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
