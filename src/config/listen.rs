use std::collections::{HashMap, HashSet};

use crate::sections::{Address, ConfigSection, Line};

use super::{Bind, Error, Server, Acl};

#[derive(Debug)]
pub struct Listen {
    pub name: String,
    pub bind: Bind,
    pub config: HashMap<String, Option<String>>,
    pub options: HashMap<String, Option<String>>,
    pub acls: HashSet<Acl>,
    pub servers: Vec<Server>,
}

impl<'a> TryFrom<&'a ConfigSection<'a>> for Listen {
    type Error = Error<'a>;

    fn try_from(entry: &'a ConfigSection<'a>) -> Result<Self, Self::Error> {
        let ConfigSection::Listen{ proxy, lines, header_addr, ..} = entry else {
            unreachable!()
        };

        let mut servers = Vec::new();
        let mut binds = Vec::new();
        let mut config = HashMap::new();
        let mut options = HashMap::new();
        let mut acls = HashSet::new();
        let mut other = Vec::new();

        for line in lines
            .iter()
            .filter(|l| !matches!(l, Line::Blank | Line::Comment(_)))
        {
            match line {
                Line::Server {
                    name, addr, option, ..
                } => servers.push(Server {
                    name: name.to_string(),
                    addr: addr.into(),
                    option: option.map(ToOwned::to_owned),
                }),
                Line::Config { key, value, .. } => {
                    config.insert(key.to_string(), value.map(ToOwned::to_owned));
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
                Line::Bind { .. } => binds.push(line),
                _other => other.push(_other),
            }
        }

        if !other.is_empty() {
            return Err(Error::WrongListenLines(other));
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

        Ok(Listen {
            name: proxy.to_string(),
            bind: Bind {
                addr: Address::from(addr),
                config: bind_config.map(ToOwned::to_owned),
            },
            config,
            servers,
            options,
            acls,
        })
    }
}

impl<'a> Listen {
    pub fn parse_multiple(entries: &'a [ConfigSection<'a>]) -> Result<Vec<Self>, Error<'a>> {
        entries
            .iter()
            .filter(|e| matches!(e, ConfigSection::Listen { .. }))
            .map(Listen::try_from)
            .collect()
    }
}
