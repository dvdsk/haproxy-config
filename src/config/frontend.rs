use std::collections::{HashMap, HashSet};

use super::{Acl, error::Error, Name, Bind, Address};
use crate::section;
use crate::section::{BackendModifier, borrowed::Section};
use crate::line::borrowed::Line;

/// Backend to which a frontend can forward messages
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

type Pair = (Name, Frontend);

impl<'a> TryFrom<&'a Section<'a>> for Pair {
    type Error = Error;

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
                Line::Backend {
                    name,
                    modifier,
                    condition,
                    ..
                } => backends.push(Backend {
                    name: (*name).to_string(),
                    modifier: modifier.clone(),
                    condition: condition.map(ToOwned::to_owned),
                }),
                Line::Bind { .. } => binds.push(line),
                wrong => other.push(wrong),
            }
        }

        if !other.is_empty() {
            return Err(Error::wrong_frontend_lines(other));
        }

        if binds.len() > 1 {
            return Err(Error::more_then_one_bind(binds));
        }

        let (addr, bind_config) = match (binds.first(), header_addr) {
            (None, None) => return Err(Error::NoBind),
            (None, Some((addr, config))) => (addr, config),
            (Some(Line::Bind { addr, value, .. }), None) => (addr, value),
            (Some(_), None) => unreachable!(),
            (Some(_), Some(_)) => return Err(Error::HeaderAndBindLine),
        };

        Ok((
            (*proxy).to_string(),
            Frontend {
                name: (*proxy).to_string(),
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
    pub(crate) fn parse_multiple(entries: &'a [section::borrowed::Section<'a>]) -> Result<HashMap<Name, Self>, Error> {
        entries
            .iter()
            .filter(|e| matches!(e, Section::Frontend { .. }))
            .map(Pair::try_from)
            .collect()
    }
}
