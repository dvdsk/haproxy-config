use std::collections::HashMap;

use super::Error;
use crate::lines::{Address, ConfigSection, Line};

#[derive(Debug)]
pub struct Server {
    name: String,
    addr: Address,
    option: Option<String>,
}

/// sockets accepting clients
#[derive(Debug)]
pub struct Backend {
    name: String,
    config: HashMap<String, Option<String>>,
    options: HashMap<String, Option<String>>,
    servers: Vec<Server>,
}

impl<'a> TryFrom<&'a ConfigSection<'a>> for Backend {
    type Error = Error<'a>;

    fn try_from(entry: &'a ConfigSection<'a>) -> Result<Self, Self::Error> {
        let ConfigSection::Backend{ proxy, lines,  ..} = entry else {
            unreachable!()
        };

        let mut config = HashMap::new();
        let mut options = HashMap::new();
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
                Line::Server {
                    name,
                    addr,
                    option,
                    ..
                } => servers.push(Server {
                    name: name.to_string(),
                    addr: addr.into(),
                    option: option.map(ToOwned::to_owned),
                }),
                _other => other.push(_other),
            }
        }

        if !other.is_empty() {
            return Err(Error::WrongFrontendLines(other));
        }

        Ok(Backend {
            name: proxy.to_string(),
            config,
            options,
            servers,
        })
    }
}

impl<'a> Backend {
    pub fn parse_multiple(entries: &'a [ConfigSection<'a>]) -> Result<Vec<Self>, Error<'a>> {
        entries
            .into_iter()
            .filter(|e| matches!(e, ConfigSection::Backend { .. }))
            .map(Backend::try_from)
            .collect()
    }
}
