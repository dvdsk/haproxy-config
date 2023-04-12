use super::super::sections::owned::Section;

use crate::sections::line::{borrowed, owned::Line};

/// Errors that can occure when transforming a list of [sections](Section) to a [Config]
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Unknown lines in the input, you should filter these out if you want to ignore them")]
    UnknownLines(Vec<String>),

    #[error("Each config must have a global section")]
    MissingGlobal,

    #[error("The following lines are not allowed in a global section: {0:#?}")]
    WrongGlobalLines(Vec<Line>),
    #[error("The following lines are not allowed in a listen section: {0:#?}")]
    WrongListenLines(Vec<Line>),
    #[error("The following lines are not allowed in a backend section: {0:#?}")]
    WrongBackendLines(Vec<Line>),
    #[error("The following lines are not allowed in a userlist section: {0:#?}")]
    WrongUserlistLines(Vec<Line>),
    #[error("The following lines are not allowed in a default section: {0:#?}")]
    WrongDefaultLines(Vec<Line>),
    #[error("The following lines are not allowed in a frontend section: {0:#?}")]
    WrongFrontendLines(Vec<Line>),

    #[error("There can only be a single global section")]
    MultipleGlobalEntries(Vec<Section>),
    #[error("There can only be a single default section")]
    MultipleDefaultEntries(Vec<Section>),

    #[error("Every ACL should have a rule, this one had none: {0}")]
    AclWithoutRule(String),

    #[error("Every frontend or listen section must bind to as address")]
    NoBind,
    #[error("A frontend or listen section may only bind a single address this section has multiple lines binding to an adress: {0:#?}")]
    MoreThenOneBind(Vec<Line>),
    #[error("A frontend or listen section may only bind a single address however both the header and a bind line further downwere found")]
    HeaderAndBindLine,

    #[error("Processes can only run as single user however multiple where specified: {0:#?}")]
    MultipleSysUsers(Vec<Line>),
    #[error("Processes can only run as single group however multiple where specified: {0:#?}")]
    MultipleSysGroups(Vec<Line>),
    #[error("The user line in a global section refers to the user haproxy runs as. It is not the same as an hapoxy user and has no group argument. Add a line \"group <some_unix_group>\" if you want to specify the unix group haproxy should run as.")]
    SysUserHasGroups(Line),
    #[error("The group line in a global section refers to the user haproxy runs as. It is not the same as an hapoxy user and has no group argument. Add a line \"user <some_unix_user>\" if you want to specify the unix user haproxy should run as.")]
    SysGroupHasUsers(Line),
}

impl Error {
    pub fn unknown_lines(lines: Vec<&str>) -> Error {
        Error::UnknownLines(lines.into_iter().map(ToString::to_string).collect())
    }
}

impl Error {
    pub fn acl_without_rule(s: &str) -> Error {
        Error::AclWithoutRule(s.to_string())
    }
}

macro_rules! from_borrowed_lines {
    ($enum_variant:ident $fn_name:ident) => {
        impl Error {
            pub fn $fn_name(lines: Vec<&borrowed::Line>) -> Error {
                Error::$enum_variant(lines.into_iter().map(|l| Line::from(l)).collect())
            }
        }
    };
}

from_borrowed_lines! {WrongGlobalLines wrong_global_lines}
from_borrowed_lines! {WrongListenLines wrong_listen_lines}
from_borrowed_lines! {WrongBackendLines wrong_backend_lines}
from_borrowed_lines! {WrongUserlistLines wrong_userlist_lines}
from_borrowed_lines! {WrongDefaultLines wrong_default_lines}
from_borrowed_lines! {WrongFrontendLines wrong_frontend_lines}
from_borrowed_lines! {MoreThenOneBind more_then_one_bind}
from_borrowed_lines! {MultipleSysUsers multiple_sys_users}
from_borrowed_lines! {MultipleSysGroups multiple_sys_groups}

macro_rules! from_borrowed_line {
    ($enum_variant:ident $fn_name:ident) => {
        impl Error {
            pub fn $fn_name(s: &borrowed::Line) -> Error {
                Error::$enum_variant(Line::from(s))
            }
        }
    };
}

from_borrowed_line! {SysUserHasGroups sys_user_has_groups}
from_borrowed_line! {SysGroupHasUsers sys_group_has_users}

use crate::sections::borrowed::Section as BorrowedSection;
macro_rules! from_borrowed_section {
    ($enum_variant:ident $fn_name:ident) => {
        impl Error {
            pub fn $fn_name(s: Vec<&BorrowedSection>) -> Error {
                Error::$enum_variant(s.into_iter().map(|s| Section::from(s)).collect())
            }
        }
    };
}
from_borrowed_section! {MultipleGlobalEntries multiple_global_entries}
from_borrowed_section! {MultipleDefaultEntries multiple_default_entries}
