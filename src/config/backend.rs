use std::collections::{HashMap, HashSet};

use super::{Acl, Error, Name, Server};
use crate::sections::{Line, Section};

/// sockets accepting clients
#[derive(Debug)]
pub struct Backend {
    pub name: String,
    pub config: HashMap<String, Option<String>>,
    pub options: HashMap<String, Option<String>>,
    pub acls: HashSet<Acl>,
    pub servers: Vec<Server>,
}

type Pair = (Name, Backend);
impl<'a> TryFrom<&'a Section<'a>> for Pair {
    type Error = Error<'a>;

    fn try_from(entry: &'a Section<'a>) -> Result<(Name, Backend), Self::Error> {
        let Section::Backend{ proxy, lines,  ..} = entry else {
            unreachable!()
        };

        let mut config = HashMap::new();
        let mut options = HashMap::new();
        let mut acls = HashSet::new();
        let mut servers = Vec::new();
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
                Line::Server {
                    name, addr, option, ..
                } => servers.push(Server {
                    name: name.to_string(),
                    addr: addr.into(),
                    option: option.map(ToOwned::to_owned),
                }),
                _other => other.push(_other),
            }
        }

        if !other.is_empty() {
            return Err(Error::WrongBackendLines(other));
        }

        Ok((
            proxy.to_string(),
            Backend {
                name: proxy.to_string(),
                config,
                options,
                acls,
                servers,
            },
        ))
    }
}

impl<'a> Backend {
    pub fn parse_multiple(entries: &'a [Section<'a>]) -> Result<HashMap<Name, Self>, Error<'a>> {
        entries
            .iter()
            .filter(|e| matches!(e, Section::Backend { .. }))
            .map(Pair::try_from)
            .collect()
    }
}
