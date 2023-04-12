use crate::config::{Address, Password};
use crate::sections::BackendModifier;

#[derive(Debug)]
pub enum Line {
    Server {
        name: String,
        addr: Address,
        option: Option<String>,
        comment: Option<String>,
    },
    Option {
        keyword: String,
        value: Option<String>,
        comment: Option<String>,
    },
    Bind {
        addr: Address,
        value: Option<String>,
        comment: Option<String>,
    },
    Acl {
        name: String,
        rule: Option<String>,
        comment: Option<String>,
    },
    Backend {
        name: String,
        modifier: Option<BackendModifier>,
        condition: Option<String>,
        comment: Option<String>,
    },
    /// This usually refers to a haproxy user group however if it is in the global section of a
    /// config it is the systemuser haproxy will try to run as after starting up.
    Group {
        name: String,
        users: Vec<String>,
        comment: Option<String>,
    },
    User {
        name: String,
        password: Password,
        groups: Vec<String>,
        comment: Option<String>,
    },
    /// A global paramater describing the system user haproxy should run as
    SysUser {
        name: String,
    },
    Config {
        key: String,
        value: Option<String>,
        comment: Option<String>,
    },
    Comment(String),
    Blank,
}
