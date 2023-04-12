use std::collections::HashMap;

use super::{error::Error, Name};
use crate::sections::{lines::borrowed::Line, PasswordRef, borrowed::Section};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Group {
    pub name: String,
    pub users: Vec<String>,
}

/// Owned variant of [PasswordRef]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Password {
    Secure(String),
    Insecure(String),
}

impl From<&PasswordRef<'_>> for Password {
    fn from(value: &PasswordRef) -> Self {
        match value {
            PasswordRef::Secure(p) => Self::Secure(p.to_string()),
            PasswordRef::Insecure(p) => Self::Insecure(p.to_string()),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct User {
    pub name: String,
    pub password: Password,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Userlist {
    pub name: String,
    pub groups: Vec<Group>,
    pub users: Vec<User>,
}

type Pair = (Name, Userlist);
impl<'a> TryFrom<&'a Section<'a>> for Pair {
    type Error = Error;

    fn try_from(entry: &'a Section<'a>) -> Result<Pair, Self::Error> {
        let Section::Userlist{name, lines, ..} = entry else {
            unreachable!()
        };

        let mut users = Vec::new();
        let mut groups = Vec::new();
        let mut other = Vec::new();

        for line in lines
            .iter()
            .filter(|l| !matches!(l, Line::Blank | Line::Comment(_)))
        {
            match line {
                Line::User { name, password, .. } => users.push(User {
                    name: name.to_string(),
                    password: Password::from(password),
                }),
                Line::Group { name, users, .. } => groups.push(Group {
                    name: name.to_string(),
                    users: users.iter().map(ToString::to_string).collect(),
                }),
                _other => other.push(_other),
            }
        }

        if !other.is_empty() {
            return Err(Error::wrong_userlist_lines(other));
        }

        Ok((
            name.to_string(),
            Userlist {
                name: name.to_string(),
                users,
                groups,
            },
        ))
    }
}

impl<'a> Userlist {
    pub fn parse_multiple(entries: &'a [Section<'a>]) -> Result<HashMap<Name, Self>, Error> {
        entries
            .iter()
            .filter(|e| matches!(e, Section::Userlist { .. }))
            .map(Pair::try_from)
            .collect()
    }
}
