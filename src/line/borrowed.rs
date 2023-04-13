use crate::section::PasswordRef;
use crate::section::BackendModifier;
use crate::section::AddressRef;

#[derive(Debug)]
pub enum Line<'input> {
    Server {
        name: &'input str,
        addr: AddressRef<'input>,
        option: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Option {
        keyword: &'input str,
        value: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Bind {
        addr: AddressRef<'input>,
        value: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Acl {
        name: &'input str,
        rule: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Backend {
        name: &'input str,
        modifier: Option<BackendModifier>,
        condition: Option<&'input str>,
        comment: Option<&'input str>,
    },
    /// This usually refers to a haproxy user group however if it is in the global section of a
    /// config it is the systemuser haproxy will try to run as after starting up.
    Group {
        name: &'input str,
        users: Vec<&'input str>,
        comment: Option<&'input str>,
    },
    User {
        name: &'input str,
        password: PasswordRef<'input>,
        groups: Vec<&'input str>,
        comment: Option<&'input str>,
    },
    /// A global paramater describing the system user haproxy should run as
    SysUser {
        name: &'input str,
    },
    Config {
        key: &'input str,
        value: Option<&'input str>,
        comment: Option<&'input str>,
    },
    Comment(&'input str),
    Blank,
}
