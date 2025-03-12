use crate::parser::{default_port, Context, Input, Parser, SchemeType};
use crate::{Host, ParseError, Position, Url};
use alloc::string::String;
use alloc::string::ToString;
#[derive(Clone)]
pub struct Url {
    /// Syntax in pseudo-BNF:
    ///
    ///   url = scheme ":" [ hierarchical | non-hierarchical ] [ "?" query ]? [ "#" fragment ]?
    ///   non-hierarchical = non-hierarchical-path
    ///   non-hierarchical-path = /* Does not start with "/" */
    ///   hierarchical = authority? hierarchical-path
    ///   authority = "//" userinfo? host [ ":" port ]?
    ///   userinfo = username [ ":" password ]? "@"
    ///   hierarchical-path = [ "/" path-segment ]+
    serialization: String,
    scheme_end: u32,
    username_end: u32,
    host_start: u32,
    host_end: u32,
    host: HostInternal,
    port: Option<u16>,
    path_start: u32,
    query_start: Option<u32>,
    fragment_start: Option<u32>,
}
#[derive(Copy, Clone, Debug)]
pub enum Position {
    BeforeScheme,
    AfterScheme,
    BeforeUsername,
    AfterUsername,
    BeforePassword,
    AfterPassword,
    BeforeHost,
    AfterHost,
    BeforePort,
    AfterPort,
    BeforePath,
    AfterPath,
    BeforeQuery,
    AfterQuery,
    BeforeFragment,
    AfterFragment,
}
#[inline]
pub fn port(url: &Url) -> &str {
    &url[Position::BeforePort..Position::AfterPort]
}
