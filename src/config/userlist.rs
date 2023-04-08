use crate::sections::{ConfigSection, Line, PasswordRef};
use super::Error;

#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub users: Vec<String>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub password: Password,
}

#[derive(Debug)]
pub struct Userlist {
    pub name: String,
    pub groups: Vec<Group>,
    pub users: Vec<User>,
}

impl<'a> TryFrom<&'a ConfigSection<'a>> for Userlist {
    type Error = Error<'a>;

    fn try_from(entry: &'a ConfigSection<'a>) -> Result<Self, Self::Error> {
        let ConfigSection::Userlist{name, lines, ..} = entry else {
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
                    password: password.into(),
                }),
                Line::Group { name, users, .. } => groups.push(Group {
                    name: name.to_string(),
                    users: users.iter().map(ToString::to_string).collect(),
                }),
                _other => other.push(_other),
            }
        }

        if !other.is_empty() {
            return Err(Error::WrongUserlistLines(other));
        }

        Ok(Userlist {
            name: name.to_string(),
            users,
            groups,
        })
    }
}

impl<'a> Userlist {
    pub fn parse_multiple(entries: &'a [ConfigSection<'a>]) -> Result<Vec<Self>, Error<'a>> {
        entries
            .iter()
            .filter(|e| matches!(e, ConfigSection::Userlist { .. }))
            .map(Userlist::try_from)
            .collect()
    }
}
