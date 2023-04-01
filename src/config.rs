// http://docs.haproxy.org/2.7/configuration.html#4.1

use itertools::{Either, Itertools};
use std::collections::HashMap;

use crate::lines::Line;

use super::lines::ConfigEntry;

pub struct Config {
    global: Global,
    default: Default,
    frontends: Frontends,
    backends: Backends,
    listen: Listens,
    userlists: Userlists,
}

impl<'a> TryFrom<&'a [ConfigEntry<'a>]> for Config {
    type Error = Error<'a>;

    fn try_from(entries: &'a[ConfigEntry<'a>]) -> Result<Self, Self::Error> {
        Ok(Config {
            global: Global::try_from(entries)?,
            default: Default::try_from(entries)?,
            frontends: Frontends::try_from(entries)?,
            backends: Backends::try_from(entries)?,
            listen: Listens::try_from(entries)?,
            userlists: Userlists::try_from(entries)?,
        })
    }
}

pub enum Error<'a> {
    MissingGlobal,
    MultipleGlobalEntries(Vec<&'a ConfigEntry<'a>>),
    WrongGlobalLines(Vec<&'a Line<'a>>),
}

pub struct Global {
    config: HashMap<String, Option<String>>,
}

impl<'a> TryFrom<&'a[ConfigEntry<'a>]> for Global {
    type Error = Error<'a>;

    fn try_from(entries: &'a[ConfigEntry<'a>]) -> Result<Self, Self::Error> {
        let global_entries: Vec<_> = entries
            .into_iter()
            .filter(|e| matches!(e, ConfigEntry::Global { .. }))
            .collect();

        if global_entries.len() > 1 {
            return Err(Error::MultipleGlobalEntries(global_entries));
        }

        let global = global_entries.first().ok_or(Error::MissingGlobal)?;
        let ConfigEntry::Global{ lines, .. } = global else { unreachable!() };

        let (config, other): (HashMap<String, Option<String>>, Vec<_>) =
            lines.iter().partition_map(|l| match l {
                Line::Config { key, value, .. } => {
                    let key = key.to_string();
                    let value = value.map(ToOwned::to_owned);
                    Either::Left((key, value))
                }
                _other => Either::Right(_other),
            });

        if !other.is_empty() {
            return Err(Error::WrongGlobalLines(other));
        }

        Ok(Global { config })
    }
}

pub struct Default;

impl<'a> TryFrom<&'a [ConfigEntry<'a>]> for Default {
    type Error = Error<'a>;

    fn try_from(entries: &[ConfigEntry<'_>]) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// sockets accepting clients
pub struct Frontends;

impl<'a> TryFrom<&'a [ConfigEntry<'a>]> for Frontends {
    type Error = Error<'a>;

    fn try_from(entries: &[ConfigEntry<'_>]) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// servers to which traffic can be forwarded
pub struct Backends;

impl<'a> TryFrom<&'a [ConfigEntry<'a>]> for Backends {
    type Error = Error<'a>;

    fn try_from(entries: &[ConfigEntry<'_>]) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// socket on which to listen and where to forward the traffic
pub struct Listens;

impl<'a> TryFrom<&'a [ConfigEntry<'a>]> for Listens {
    type Error = Error<'a>;

    fn try_from(entries: &[ConfigEntry<'_>]) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct Userlists;

impl<'a> TryFrom<&'a [ConfigEntry<'a>]> for Userlists {
    type Error = Error<'a>;

    fn try_from(entries: &[ConfigEntry<'_>]) -> Result<Self, Self::Error> {
        todo!()
    }
}
