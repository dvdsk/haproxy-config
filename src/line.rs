use crate::config::{Address, Password};

/// Zero copy representations of the lines in a config section. 
pub mod borrowed;
/// Owned representations of the lines in a config section, far easier to work with. Can be created
/// from a [`borrowed::Line`] using [`From`]
pub mod owned;

impl<'a> From<&'a borrowed::Line<'a>> for owned::Line {
    fn from(line: &'a borrowed::Line<'a>) -> owned::Line {
        match line {
            borrowed::Line::Server {
                name,
                addr,
                option,
                comment,
            } => owned::Line::Server {
                name: (*name).to_string(),
                addr: Address::from(addr),
                option: option.map(ToOwned::to_owned),
                comment: comment.map(ToOwned::to_owned),
            },
            borrowed::Line::Option {
                keyword,
                value,
                comment,
            } => owned::Line::Option {
                keyword: (*keyword).to_string(),
                value: value.map(ToOwned::to_owned),
                comment: comment.map(ToOwned::to_owned),
            },
            borrowed::Line::Bind {
                addr,
                value,
                comment,
            } => owned::Line::Bind {
                addr: Address::from(addr),
                value: value.map(ToOwned::to_owned),
                comment: comment.map(ToOwned::to_owned),
            },
            borrowed::Line::Acl {
                name,
                rule,
                comment,
            } => owned::Line::Acl {
                name: (*name).to_string(),
                rule: rule.map(ToOwned::to_owned),
                comment: comment.map(ToOwned::to_owned),
            },
            borrowed::Line::Backend {
                name,
                modifier,
                condition,
                comment,
            } => owned::Line::Backend {
                name: (*name).to_string(),
                modifier: modifier.clone(),
                condition: condition.map(ToOwned::to_owned),
                comment: comment.map(ToOwned::to_owned),
            },
            borrowed::Line::Group {
                name,
                users,
                comment,
            } => owned::Line::Group {
                name: (*name).to_string(),
                users: users.iter().map(|s| (*s).to_string()).collect(),
                comment: comment.map(ToOwned::to_owned),
            },
            borrowed::Line::User {
                name,
                password,
                groups,
                comment,
            } => owned::Line::User {
                name: (*name).to_string(),
                password: Password::from(password),
                groups: groups.iter().map(|s| (*s).to_string()).collect(),
                comment: comment.map(ToOwned::to_owned),
            },
            borrowed::Line::SysUser { name } => owned::Line::SysUser {
                name: (*name).to_string(),
            },
            borrowed::Line::Config {
                key,
                value,
                comment,
            } => owned::Line::Config {
                key: (*key).to_string(),
                value: value.map(ToOwned::to_owned),
                comment: comment.map(ToOwned::to_owned),
            },
            borrowed::Line::Comment(s) => owned::Line::Comment((*s).to_string()),
            borrowed::Line::Blank => owned::Line::Blank,
        }
    }
}
