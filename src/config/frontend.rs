use std::collections::{HashMap, HashSet};

use super::Error;
use crate::lines::{Address, BackendModifier, ConfigSection, Line};

#[derive(Debug)]
pub struct Backend {
    name: String,
    modifier: Option<BackendModifier>,
    condition: Option<String>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Acl {
    name: String,
    rule: String,
}

#[derive(Debug)]
pub struct Bind {
    addr: Address,
    config: Option<String>,
}

/// sockets accepting clients
#[derive(Debug)]
pub struct Frontend {
    name: String,
    config: HashMap<String, Option<String>>,
    options: HashMap<String, Option<String>>,
    acls: HashSet<Acl>,
    backends: Vec<Backend>,
    bind: Bind,
}

impl<'a> TryFrom<&'a ConfigSection<'a>> for Frontend {
    type Error = Error<'a>;

    fn try_from(entry: &'a ConfigSection<'a>) -> Result<Self, Self::Error> {
        let ConfigSection::Frontend{ proxy, lines, header_addr, ..} = entry else {
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
                        rule: rule.ok_or(Error::AclWithoutRule(*name))?.to_string(),
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

        Ok(Frontend {
            name: proxy.to_string(),
            config,
            options,
            acls,
            backends,
            bind: Bind {
                addr: Address::from(addr),
                config: bind_config.map(ToOwned::to_owned),
            },
        })
    }
}

impl<'a> Frontend {
    pub fn parse_multiple(entries: &'a [ConfigSection<'a>]) -> Result<Vec<Self>, Error<'a>> {
        entries
            .into_iter()
            .filter(|e| matches!(e, ConfigSection::Frontend { .. }))
            .map(Frontend::try_from)
            .collect()
    }
}
