use std::net::Ipv4Addr;

use super::error::Error;
use super::sections::*;

/// # Examples
/// ```
/// # use haproxy_config_parser::parse_sections;
/// let file = include_str!("../tests/medium_haproxy.cfg");
/// if let Err(e) = parse_sections(file) {
///     e.print();
///     panic!("{}", e.inner);
/// }
/// ```
pub fn parse_sections(input: &str) -> Result<Vec<ConfigSection>, Error<'_>> {
    parser::configuration(input).map_err(|e| Error {
        inner: e,
        source: input,
        path: None,
    })
}

peg::parser! {
    grammar parser() for str {
        pub(super) rule configuration() -> Vec<ConfigSection<'input>>
            = (config_comment() / config_blank() / global_section() / defaults_section() / userlist_section() / listen_section() / frontend_section() / backend_section()/ unknown_line())*

        rule unknown_line() -> ConfigSection<'input>
            = line:$([^ '\n']+) line_break() {
                ConfigSection::UnknownLine{ line }
            }

        pub(super) rule global_section() -> ConfigSection<'input>
            = comment:global_header() lines:config_block() {
                ConfigSection::Global{ comment, lines }
            }

        rule defaults_section() -> ConfigSection<'input>
            = h:defaults_header() lines:config_block() {
                ConfigSection::Default{ comment: h.1, proxy: h.0, lines }
            }

        rule userlist_section() -> ConfigSection<'input>
            = h:userlist_header() lines:config_block() {
                ConfigSection::Userlist{ comment: h.1, name: h.0 , lines}
            }

        rule listen_section() -> ConfigSection<'input>
            = h:listen_header() lines:config_block() {
                ConfigSection::Listen{ comment: h.1, proxy: h.0, header_addr: h.2, lines}
            }

        rule frontend_section() -> ConfigSection<'input>
            = h:frontend_header() lines:config_block() {
                ConfigSection::Frontend{ comment: h.1, proxy: h.0, header_addr: h.2, lines }
            }

        rule backend_section() -> ConfigSection<'input>
            = h:backend_header() lines:config_block() {
                ConfigSection::Backend{ comment: h.1, proxy: h.0 , lines}
            }

        rule global_header() -> Option<&'input str>
            = whitespace() "global" whitespace() c:comment_text()? line_break() { c }

        rule userlist_header() -> (&'input str, Option<&'input str>)
            = whitespace() "userlist" whitespace() p:proxy_name() c:comment_text()? line_break() {(p,c)}

        rule defaults_header() -> (Option<&'input str>, Option<&'input str>)
            = whitespace() "defaults" whitespace() p:proxy_name()? whitespace() c:comment_text()? line_break() {(p,c)}

        rule header_bind() -> (AddressRef<'input>, Option<&'input str>)
            = s:service_address() v:value()? {(s, v)}

        rule listen_header() -> (&'input str, Option<&'input str>, Option<(AddressRef<'input>, Option<&'input str>)>)
            = whitespace() "listen" whitespace() p:proxy_name() whitespace() hb:header_bind()? c:comment_text()? line_break() {(p, c, hb)}

        rule frontend_header() -> (&'input str, Option<&'input str>, Option<(AddressRef<'input>, Option<&'input str>)>)
            = whitespace() "frontend" whitespace() p:proxy_name() whitespace() hb:header_bind()? c:comment_text()? line_break() {(p, c, hb)}

        pub(super) rule backend_header() -> (&'input str, Option<&'input str>)
            = whitespace() "backend" whitespace() p:proxy_name() whitespace() value()? c:comment_text()? line_break() {(p,c)}

        rule config_block() -> Vec<Line<'input>>
            = e:(server_line() / option_line() / bind_line() / acl_line() / backend_line() / group_line() / user_line() / config_line() / comment_line() / blank_line())* { e }

        rule server_line() -> Line<'input>
            = whitespace() "server" whitespace() name:server_name() whitespace() addr:service_address() option:value()? comment:comment_text()? line_break() eof()? {
                // let option = option.map(str::trim);
                Line::Server { name, addr, option, comment }
            }

        rule option_line() -> Line<'input>
            = whitespace() "option" whitespace() keyword:keyword() value:value()? comment:comment_text()? line_break() eof()? {
                Line::Option { keyword, value, comment }
            }

        rule bind_line() -> Line<'input>
            = whitespace() "bind" whitespaceplus() addr:service_address() value:value()? comment:comment_text()? line_break() eof()? {
                Line::Bind { addr, value, comment }
            }

        rule acl_line() -> Line<'input>
        = whitespace() "acl" whitespace() name:acl_name() r:value()? comment:comment_text()? line_break() eof()? {
            Line::Acl { name, rule: r, comment }
        }

        rule modifier() -> BackendModifier
        = "if" { BackendModifier::If } / "unless" { BackendModifier::Unless }

        rule backend_line() -> Line<'input>
            = whitespace() ("use_backend" / "default_backend") whitespace() name:backend_name() whitespace() modifier:modifier()? whitespace() condition:backend_condition()? comment:comment_text()? line_break() eof()? {
                Line::Backend {name, modifier, condition, comment }
            }

        rule users() -> Vec<&'input str>
            = "users" users:(value() ++ whitespaceplus()) {
                let mut users = users;
                for user in &mut users {
                    *user = user.trim();
                }
                users
            }

        pub(super) rule group_line() -> Line<'input>
            = whitespace() "group" whitespace() name:group_name() whitespace() users:users() comment:comment_text()? line_break() eof()? {
                Line::Group { name, users, comment }
            }

        rule password_type() -> bool
            = "password" { true } / "insecure-password" { false }

        rule groups() -> Vec<&'input str>
            = whitespaceplus() "groups" groups:(value() ++ whitespaceplus()) {
                let mut groups = groups;
                for group in &mut groups {
                    *group = group.trim();
                }
                groups
            }

        pub(super) rule user_line() -> Line<'input>
            = whitespace() "user" whitespace() name:user_name() whitespace() secure:password_type() whitespaceplus() password:password() groups:groups()? comment:comment_text()? line_break() eof()? {
                let password = if secure {
                    PasswordRef::Secure(password)
                } else {
                    PasswordRef::Insecure(password)
                };
                let groups = groups.unwrap_or_default();
                Line::User { name, password, groups, comment}
            }

        pub(super) rule config_line() -> Line<'input>
            = whitespace() !("defaults" / "global" / "userlist" / "listen" / "frontend" / "backend" / "server") key:keyword() value:value()? comment:comment_text()? line_break() eof()? {
                Line::Config { key, value, comment }
            }

        rule config_comment() -> ConfigSection<'input>
            = whitespace() t:comment_text() line_break() eof()? { ConfigSection::Comment(t) }

        rule comment_line() -> Line<'input>
            = whitespace() t:comment_text() line_break() eof()? { Line::Comment(t) }

        rule blank_line() -> Line<'input>
            = whitespace() line_break() eof()? { Line::Blank }

        rule config_blank() -> ConfigSection<'input>
            = whitespace() line_break() eof()? { ConfigSection::BlankLine }

        pub(super) rule comment_text() -> &'input str
            = "#" s:$(char()*) &(line_break() / eof()) { s }

        rule line_break()
            = quiet!{['\n']}

        rule eof()
            = quiet!{![_]}

        rule keyword() -> &'input str
            = $((("errorfile" / "timeout") whitespace())? ['a'..='z' | '0'..='9' | '-' | '_' | '.']+)

        rule alphanumeric_plus() -> &'input str
            = $(['a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | ':']+)

        rule server_name() -> &'input str
            = alphanumeric_plus()

        rule acl_name() -> &'input str
            = alphanumeric_plus()

        rule backend_name() -> &'input str
            = alphanumeric_plus()

        rule group_name() -> &'input str
            = alphanumeric_plus()

        rule user_name() -> &'input str
            = alphanumeric_plus()

        rule not_comment_or_end() -> &'input str
            = $([^ '#' | '\n']+)

        rule password() -> &'input str
            = $([^ '#' | '\n' | ' ']+)

        rule backend_condition() -> &'input str
            = not_comment_or_end()

        rule service_address() -> AddressRef<'input>
            = host:host() [':']? port:port()? {
                AddressRef {host, port}
            }

        rule host() -> HostRef<'input>
            = ipv4_host() / dns_host() / wildcard_host()

        rule port() -> u16
            = p:$(['0'..='9']+) { p.parse().expect("port must fit in a u16") }

        rule digits_u8() -> u8
            = d:$(['0'..='9']*<1,3>) {
                d.parse().expect("digits must represent unsigned 8 bit integer")
            }

        rule ipv4_host() -> HostRef<'input>
            = a:digits_u8() "." b:digits_u8() "." c:digits_u8() "." d:digits_u8() {
                HostRef::Ipv4(Ipv4Addr::new(a,b,c,d))
            }

        rule dns_host() -> HostRef<'input>
            = s:$(['a'..='z' | 'A'..='Z' | '-' | '.' | '0'..='9']+) { HostRef::Dns(s) }

        rule wildcard_host() -> HostRef<'input>
            = "*" { HostRef::Wildcard }

        rule proxy_name() -> &'input str
            = alphanumeric_plus()

        rule value() -> &'input str
            = whitespaceplus() s:not_comment_or_end() { s }

        rule char()
            = [^ '\n']

        pub(super) rule whitespace()
            = quiet!{[' ' | '\t']*}

        rule whitespaceplus()
            = quiet!{[' ' | '\t']+}

    }
}

#[cfg(test)]
mod tests {
    use super::parser;
    use crate::sections::{Line, PasswordRef};

    #[test]
    fn global() {
        parser::configuration(include_str!("global_section.txt")).unwrap();
    }

    #[test]
    fn config_line() {
        parser::config_line(include_str!("config_line.txt")).unwrap();
    }

    #[test]
    fn backend_with_comment() {
        parser::backend_header(include_str!("backend_with_comment.txt")).unwrap();
    }

    #[test]
    fn whitespace() {
        let four_spaces = "    ";
        parser::whitespace(four_spaces).unwrap();
    }

    #[test]
    fn comment_text() {
        parser::comment_text("# testing comment_text, *=* () hi!").unwrap();
    }

    #[test]
    fn user_with_group() {
        let line = parser::user_line(include_str!("user_with_group.txt")).unwrap();
        match line {
            Line::User { groups, .. } if groups == vec!["G1"] => (),
            _ => panic!("groups not correct, line: {:?}", line),
        }
    }

    #[test]
    fn user() {
        let line = parser::user_line(include_str!("user.txt")).unwrap();
        match line {
            Line::User {
                groups,
                password: PasswordRef::Insecure(pass),
                ..
            } => {
                assert_eq!(groups, Vec::<&str>::new());
                assert_eq!(pass, "test");
            }
            _ => panic!("user not correct, line: {:?}", line),
        }
    }

    #[test]
    fn group_with_users() {
        let line = parser::group_line(include_str!("group_with_users.txt")).unwrap();
        match line {
            Line::Group { users, .. } if users == vec!["haproxy"] => (),
            _ => panic!("groups not correct, line: {:?}", line),
        }
    }
}
