use std::collections::{HashMap, HashSet};

use super::Address;
use super::{Acl, error::Error, Name, Server};
use crate::section::borrowed::Section;
use crate::line::borrowed::Line; 

/// Frontends specify whether to forward traffic here 
/// using [`config::backends`][super::frontend::Backend].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backend {
    pub name: String,
    pub config: HashMap<String, Option<String>>,
    pub options: HashMap<String, Option<String>>,
    pub acls: HashSet<Acl>,
    pub servers: Vec<Server>,
}

type Pair = (Name, Backend);
impl<'a> TryFrom<&'a Section<'a>> for Pair {
    type Error = Error;

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
                Line::Acl { name, rule, .. } => {
                    acls.insert(Acl {
                        name: (*name).to_string(),
                        rule: rule.ok_or(Error::acl_without_rule(name))?.to_string(),
                    });
                }
                Line::Server {
                    name, addr, option, ..
                } => servers.push(Server {
                    name: (*name).to_string(),
                    addr: Address::from(addr),
                    option: option.map(ToOwned::to_owned),
                }),
                wrong => other.push(wrong),
            }
        }

        if !other.is_empty() {
            return Err(Error::wrong_backend_lines(other));
        }

        Ok((
            (*proxy).to_string(),
            Backend {
                name: (*proxy).to_string(),
                config,
                options,
                acls,
                servers,
            },
        ))
    }
}

impl<'a> Backend {
    pub(crate) fn parse_multiple(entries: &'a [Section<'a>]) -> Result<HashMap<Name, Self>, Error> {
        entries
            .iter()
            .filter(|e| matches!(e, Section::Backend { .. }))
            .map(Pair::try_from)
            .collect()
    }
}
