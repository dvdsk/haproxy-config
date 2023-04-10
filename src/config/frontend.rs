use std::collections::{HashMap, HashSet};

use super::{Acl, Error, Name, Bind, Address};
use crate::sections::{BackendModifier, Line, Section};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Backend {
    pub name: String,
    pub modifier: Option<BackendModifier>,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frontend {
    pub name: String,
    pub config: HashMap<String, Option<String>>,
    pub options: HashMap<String, Option<String>>,
    pub acls: HashSet<Acl>,
    pub backends: Vec<Backend>,
    pub bind: Bind,
}

pub type Pair = (Name, Frontend);

impl<'a> TryFrom<&'a Section<'a>> for Pair {
    type Error = Error<'a>;

    fn try_from(entry: &'a Section<'a>) -> Result<Self, Self::Error> {
        let Section::Frontend{ proxy, lines, header_addr, ..} = entry else {
            unreachable!()
        };

        let mut config = HashMap::new();
        let mut options = HashMap::new();
        let mut acls = HashSet::new();
        let mut backends = Vec::new();
        let mut binds = Vec::new();
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
                Line::Acl { name, rule, .. } => {
                    acls.insert(Acl {
                        name: name.to_string(),
                        rule: rule.ok_or(Error::AclWithoutRule(name))?.to_string(),
                    });
                }
                Line::Backend {
                    name,
                    modifier,
                    condition,
                    ..
                } => backends.push(Backend {
                    name: name.to_string(),
                    modifier: modifier.clone(),
                    condition: condition.map(ToOwned::to_owned),
                }),
                Line::Bind { .. } => binds.push(line),
                _other => other.push(_other),
            }
        }

        if !other.is_empty() {
            return Err(Error::WrongFrontendLines(other));
        }

        if binds.len() > 1 {
            return Err(Error::MoreThenOneBind(binds));
        }

        let (addr, bind_config) = match (binds.first(), header_addr) {
            (None, None) => return Err(Error::NoBind),
            (None, Some((addr, config))) => (addr, config),
            (Some(Line::Bind { addr, value, .. }), None) => (addr, value),
            (Some(_), None) => unreachable!(),
            (Some(_), Some(_)) => return Err(Error::HeaderAndBindLine),
        };

        Ok((
            proxy.to_string(),
            Frontend {
                name: proxy.to_string(),
                config,
                options,
                acls,
                backends,
                bind: Bind {
                    addr: Address::from(addr),
                    config: bind_config.map(ToOwned::to_owned),
                },
            },
        ))
    }
}

impl<'a> Frontend {
    pub fn parse_multiple(entries: &'a [Section<'a>]) -> Result<HashMap<Name, Self>, Error<'a>> {
        entries
            .iter()
            .filter(|e| matches!(e, Section::Frontend { .. }))
            .map(Pair::try_from)
            .collect()
    }
}
