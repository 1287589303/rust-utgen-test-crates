pub use form_urlencoded;
use crate::host::HostInternal;
use crate::net::IpAddr;
#[cfg(feature = "std")]
#[cfg(any(unix, windows, target_os = "redox", target_os = "wasi", target_os = "hermit"))]
use crate::net::{SocketAddr, ToSocketAddrs};
use crate::parser::{to_u32, Context, Parser, SchemeType, USERINFO};
use alloc::borrow::ToOwned;
use alloc::str;
use alloc::string::{String, ToString};
use core::borrow::Borrow;
use core::convert::TryFrom;
use core::fmt::Write;
use core::ops::{Range, RangeFrom, RangeTo};
use core::{cmp, fmt, hash, mem};
use percent_encoding::utf8_percent_encode;
#[cfg(feature = "std")]
#[cfg(any(unix, windows, target_os = "redox", target_os = "wasi", target_os = "hermit"))]
use std::io;
#[cfg(feature = "std")]
use std::path::{Path, PathBuf};
pub use crate::host::Host;
pub use crate::origin::{OpaqueOrigin, Origin};
pub use crate::parser::{ParseError, SyntaxViolation};
pub use crate::path_segments::PathSegmentsMut;
pub use crate::slicing::Position;
pub use form_urlencoded::EncodingOverride;
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
#[derive(Clone, Debug)]
pub struct Input<'i> {
    chars: str::Chars<'i>,
}
pub struct Parser<'a> {
    pub serialization: String,
    pub base_url: Option<&'a Url>,
    pub query_encoding_override: EncodingOverride<'a>,
    pub violation_fn: Option<&'a dyn Fn(SyntaxViolation)>,
    pub context: Context,
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SchemeType {
    File,
    SpecialNotFile,
    NotSpecial,
}
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum HostInternal {
    None,
    Domain,
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
}
impl Url {
    #[inline]
    pub fn parse(input: &str) -> Result<Url, crate::ParseError> {}
    #[inline]
    pub fn parse_with_params<I, K, V>(
        input: &str,
        iter: I,
    ) -> Result<Url, crate::ParseError>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {}
    fn strip_trailing_spaces_from_opaque_path(&mut self) {}
    #[inline]
    pub fn join(&self, input: &str) -> Result<Url, crate::ParseError> {}
    pub fn make_relative(&self, url: &Url) -> Option<String> {}
    pub fn options<'a>() -> ParseOptions<'a> {}
    #[inline]
    pub fn as_str(&self) -> &str {}
    #[inline]
    #[deprecated(since = "2.3.0", note = "use Into<String>")]
    pub fn into_string(self) -> String {}
    pub fn check_invariants(&self) -> Result<(), String> {}
    #[inline]
    pub fn origin(&self) -> Origin {}
    #[inline]
    pub fn scheme(&self) -> &str {
        self.slice(..self.scheme_end)
    }
    pub fn is_special(&self) -> bool {}
    #[inline]
    pub fn has_authority(&self) -> bool {
        debug_assert!(self.byte_at(self.scheme_end) == b':');
        self.slice(self.scheme_end..).starts_with("://")
    }
    pub fn authority(&self) -> &str {}
    #[inline]
    pub fn cannot_be_a_base(&self) -> bool {}
    pub fn username(&self) -> &str {}
    pub fn password(&self) -> Option<&str> {}
    pub fn has_host(&self) -> bool {
        !matches!(self.host, HostInternal::None)
    }
    pub fn host_str(&self) -> Option<&str> {}
    pub fn host(&self) -> Option<Host<&str>> {}
    pub fn domain(&self) -> Option<&str> {}
    #[inline]
    pub fn port(&self) -> Option<u16> {
        self.port
    }
    #[inline]
    pub fn port_or_known_default(&self) -> Option<u16> {}
    #[cfg(feature = "std")]
    #[cfg(
        any(unix, windows, target_os = "redox", target_os = "wasi", target_os = "hermit")
    )]
    pub fn socket_addrs(
        &self,
        default_port_number: impl Fn() -> Option<u16>,
    ) -> io::Result<alloc::vec::Vec<SocketAddr>> {}
    pub fn path(&self) -> &str {}
    pub fn path_segments(&self) -> Option<str::Split<'_, char>> {}
    pub fn query(&self) -> Option<&str> {}
    #[inline]
    pub fn query_pairs(&self) -> form_urlencoded::Parse<'_> {}
    pub fn fragment(&self) -> Option<&str> {}
    fn mutate<F: FnOnce(&mut Parser<'_>) -> R, R>(&mut self, f: F) -> R {}
    pub fn set_fragment(&mut self, fragment: Option<&str>) {}
    fn take_fragment(&mut self) -> Option<String> {}
    fn restore_already_parsed_fragment(&mut self, fragment: Option<String>) {}
    pub fn set_query(&mut self, query: Option<&str>) {}
    pub fn query_pairs_mut(&mut self) -> form_urlencoded::Serializer<'_, UrlQuery<'_>> {}
    fn take_after_path(&mut self) -> String {}
    pub fn set_path(&mut self, mut path: &str) {}
    #[allow(clippy::result_unit_err)]
    pub fn path_segments_mut(&mut self) -> Result<PathSegmentsMut<'_>, ()> {}
    fn restore_after_path(&mut self, old_after_path_position: u32, after_path: &str) {}
    #[allow(clippy::result_unit_err)]
    pub fn set_port(&mut self, mut port: Option<u16>) -> Result<(), ()> {
        if !self.has_host() || self.host() == Some(Host::Domain(""))
            || self.scheme() == "file"
        {
            return Err(());
        }
        if port.is_some() && port == parser::default_port(self.scheme()) {
            port = None;
        }
        self.set_port_internal(port);
        Ok(())
    }
    fn set_port_internal(&mut self, port: Option<u16>) {}
    pub fn set_host(&mut self, host: Option<&str>) -> Result<(), ParseError> {}
    fn set_host_internal(
        &mut self,
        host: Host<String>,
        opt_new_port: Option<Option<u16>>,
    ) {}
    #[allow(clippy::result_unit_err)]
    pub fn set_ip_host(&mut self, address: IpAddr) -> Result<(), ()> {}
    #[allow(clippy::result_unit_err)]
    pub fn set_password(&mut self, password: Option<&str>) -> Result<(), ()> {}
    #[allow(clippy::result_unit_err)]
    pub fn set_username(&mut self, username: &str) -> Result<(), ()> {}
    #[allow(clippy::result_unit_err, clippy::suspicious_operation_groupings)]
    pub fn set_scheme(&mut self, scheme: &str) -> Result<(), ()> {
        let mut parser = Parser::for_setter(String::new());
        let remaining = parser.parse_scheme(parser::Input::new_no_trim(scheme))?;
        let new_scheme_type = SchemeType::from(&parser.serialization);
        let old_scheme_type = SchemeType::from(self.scheme());
        if (new_scheme_type.is_special() && !old_scheme_type.is_special())
            || (!new_scheme_type.is_special() && old_scheme_type.is_special())
            || (new_scheme_type.is_file() && self.has_authority())
        {
            return Err(());
        }
        if !remaining.is_empty() || (!self.has_host() && new_scheme_type.is_special()) {
            return Err(());
        }
        let old_scheme_end = self.scheme_end;
        let new_scheme_end = to_u32(parser.serialization.len()).unwrap();
        let adjust = |index: &mut u32| {
            *index -= old_scheme_end;
            *index += new_scheme_end;
        };
        self.scheme_end = new_scheme_end;
        adjust(&mut self.username_end);
        adjust(&mut self.host_start);
        adjust(&mut self.host_end);
        adjust(&mut self.path_start);
        if let Some(ref mut index) = self.query_start {
            adjust(index)
        }
        if let Some(ref mut index) = self.fragment_start {
            adjust(index)
        }
        parser.serialization.push_str(self.slice(old_scheme_end..));
        self.serialization = parser.serialization;
        let previous_port = self.port();
        let _ = self.set_port(previous_port);
        Ok(())
    }
    #[cfg(
        all(
            feature = "std",
            any(
                unix,
                windows,
                target_os = "redox",
                target_os = "wasi",
                target_os = "hermit"
            )
        )
    )]
    #[allow(clippy::result_unit_err)]
    pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<Url, ()> {}
    #[cfg(
        all(
            feature = "std",
            any(
                unix,
                windows,
                target_os = "redox",
                target_os = "wasi",
                target_os = "hermit"
            )
        )
    )]
    #[allow(clippy::result_unit_err)]
    pub fn from_directory_path<P: AsRef<std::path::Path>>(path: P) -> Result<Url, ()> {}
    #[cfg(feature = "serde")]
    #[deny(unused)]
    pub fn serialize_internal<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {}
    #[cfg(feature = "serde")]
    #[deny(unused)]
    pub fn deserialize_internal<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Deserialize, Error};
        let (
            serialization,
            scheme_end,
            username_end,
            host_start,
            host_end,
            host,
            port,
            path_start,
            query_start,
            fragment_start,
        ) = Deserialize::deserialize(deserializer)?;
        let url = Url {
            serialization,
            scheme_end,
            username_end,
            host_start,
            host_end,
            host,
            port,
            path_start,
            query_start,
            fragment_start,
        };
        if cfg!(debug_assertions) {
            url.check_invariants().map_err(Error::custom)?
        }
        Ok(url)
    }
    #[inline]
    #[cfg(
        all(
            feature = "std",
            any(
                unix,
                windows,
                target_os = "redox",
                target_os = "wasi",
                target_os = "hermit"
            )
        )
    )]
    #[allow(clippy::result_unit_err)]
    pub fn to_file_path(&self) -> Result<PathBuf, ()> {}
    #[inline]
    fn slice<R>(&self, range: R) -> &str
    where
        R: RangeArg,
    {}
    #[inline]
    fn byte_at(&self, i: u32) -> u8 {}
}
impl<'i> Input<'i> {
    pub fn new_no_trim(input: &'i str) -> Self {
        Input { chars: input.chars() }
    }
    pub fn new_trim_tab_and_newlines(
        original_input: &'i str,
        vfn: Option<&dyn Fn(SyntaxViolation)>,
    ) -> Self {
        let input = original_input.trim_matches(ascii_tab_or_new_line);
        if let Some(vfn) = vfn {
            if input.len() < original_input.len() {
                vfn(SyntaxViolation::C0SpaceIgnored)
            }
            if input.chars().any(ascii_tab_or_new_line) {
                vfn(SyntaxViolation::TabOrNewlineIgnored)
            }
        }
        Input { chars: input.chars() }
    }
    pub fn new_trim_c0_control_and_space(
        original_input: &'i str,
        vfn: Option<&dyn Fn(SyntaxViolation)>,
    ) -> Self {
        let input = original_input.trim_matches(c0_control_or_space);
        if let Some(vfn) = vfn {
            if input.len() < original_input.len() {
                vfn(SyntaxViolation::C0SpaceIgnored)
            }
            if input.chars().any(ascii_tab_or_new_line) {
                vfn(SyntaxViolation::TabOrNewlineIgnored)
            }
        }
        Input { chars: input.chars() }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.clone().next().is_none()
    }
    #[inline]
    fn starts_with<P: Pattern>(&self, p: P) -> bool {}
    #[inline]
    pub fn split_prefix<P: Pattern>(&self, p: P) -> Option<Self> {
        let mut remaining = self.clone();
        if p.split_prefix(&mut remaining) { Some(remaining) } else { None }
    }
    #[inline]
    fn split_first(&self) -> (Option<char>, Self) {
        let mut remaining = self.clone();
        (remaining.next(), remaining)
    }
    #[inline]
    fn count_matching<F: Fn(char) -> bool>(&self, f: F) -> (u32, Self) {
        let mut count = 0;
        let mut remaining = self.clone();
        loop {
            let mut input = remaining.clone();
            if matches!(input.next(), Some(c) if f(c)) {
                remaining = input;
                count += 1;
            } else {
                return (count, remaining);
            }
        }
    }
    #[inline]
    fn next_utf8(&mut self) -> Option<(char, &'i str)> {}
}
impl SchemeType {
    pub fn is_special(&self) -> bool {
        !matches!(* self, SchemeType::NotSpecial)
    }
    pub fn is_file(&self) -> bool {
        matches!(* self, SchemeType::File)
    }
}
impl<'a> Parser<'a> {
    fn log_violation(&self, v: SyntaxViolation) {}
    fn log_violation_if(&self, v: SyntaxViolation, test: impl FnOnce() -> bool) {}
    pub fn for_setter(serialization: String) -> Parser<'a> {
        Parser {
            serialization,
            base_url: None,
            query_encoding_override: None,
            violation_fn: None,
            context: Context::Setter,
        }
    }
    pub fn parse_url(mut self, input: &str) -> ParseResult<Url> {}
    pub fn parse_scheme<'i>(&mut self, mut input: Input<'i>) -> Result<Input<'i>, ()> {
        if !input.starts_with(ascii_alpha) {
            return Err(());
        }
        debug_assert!(self.serialization.is_empty());
        while let Some(c) = input.next() {
            match c {
                'a'..='z' | '0'..='9' | '+' | '-' | '.' => self.serialization.push(c),
                'A'..='Z' => self.serialization.push(c.to_ascii_lowercase()),
                ':' => return Ok(input),
                _ => {
                    self.serialization.clear();
                    return Err(());
                }
            }
        }
        if self.context == Context::Setter {
            Ok(input)
        } else {
            self.serialization.clear();
            Err(())
        }
    }
    fn parse_with_scheme(mut self, input: Input<'_>) -> ParseResult<Url> {}
    fn parse_non_special(
        mut self,
        input: Input<'_>,
        scheme_type: SchemeType,
        scheme_end: u32,
    ) -> ParseResult<Url> {}
    fn parse_file(
        mut self,
        input: Input<'_>,
        scheme_type: SchemeType,
        base_file_url: Option<&Url>,
    ) -> ParseResult<Url> {}
    fn parse_relative(
        mut self,
        input: Input<'_>,
        scheme_type: SchemeType,
        base_url: &Url,
    ) -> ParseResult<Url> {}
    fn after_double_slash(
        mut self,
        input: Input<'_>,
        scheme_type: SchemeType,
        scheme_end: u32,
    ) -> ParseResult<Url> {}
    fn parse_userinfo<'i>(
        &mut self,
        mut input: Input<'i>,
        scheme_type: SchemeType,
    ) -> ParseResult<(u32, Input<'i>)> {}
    fn parse_host_and_port<'i>(
        &mut self,
        input: Input<'i>,
        scheme_end: u32,
        scheme_type: SchemeType,
    ) -> ParseResult<(u32, HostInternal, Option<u16>, Input<'i>)> {}
    pub fn parse_host(
        mut input: Input<'_>,
        scheme_type: SchemeType,
    ) -> ParseResult<(Host<String>, Input<'_>)> {}
    fn get_file_host(input: Input<'_>) -> ParseResult<(Host<String>, Input<'_>)> {}
    fn parse_file_host<'i>(
        &mut self,
        input: Input<'i>,
    ) -> ParseResult<(bool, HostInternal, Input<'i>)> {}
    pub fn file_host(input: Input) -> ParseResult<(bool, String, Input)> {}
    pub fn parse_port<P>(
        mut input: Input<'_>,
        default_port: P,
        context: Context,
    ) -> ParseResult<(Option<u16>, Input<'_>)>
    where
        P: Fn() -> Option<u16>,
    {}
    pub fn parse_path_start<'i>(
        &mut self,
        scheme_type: SchemeType,
        has_host: &mut bool,
        input: Input<'i>,
    ) -> Input<'i> {}
    pub fn parse_path<'i>(
        &mut self,
        scheme_type: SchemeType,
        has_host: &mut bool,
        path_start: usize,
        mut input: Input<'i>,
    ) -> Input<'i> {}
    fn last_slash_can_be_removed(serialization: &str, path_start: usize) -> bool {}
    fn shorten_path(&mut self, scheme_type: SchemeType, path_start: usize) {}
    fn pop_path(&mut self, scheme_type: SchemeType, path_start: usize) {}
    pub fn parse_cannot_be_a_base_path<'i>(
        &mut self,
        mut input: Input<'i>,
    ) -> Input<'i> {}
    #[allow(clippy::too_many_arguments)]
    fn with_query_and_fragment(
        mut self,
        scheme_type: SchemeType,
        scheme_end: u32,
        username_end: u32,
        host_start: u32,
        host_end: u32,
        host: HostInternal,
        port: Option<u16>,
        mut path_start: u32,
        remaining: Input<'_>,
    ) -> ParseResult<Url> {}
    fn parse_query_and_fragment(
        &mut self,
        scheme_type: SchemeType,
        scheme_end: u32,
        mut input: Input<'_>,
    ) -> ParseResult<(Option<u32>, Option<u32>)> {}
    pub fn parse_query<'i>(
        &mut self,
        scheme_type: SchemeType,
        scheme_end: u32,
        input: Input<'i>,
    ) -> Option<Input<'i>> {}
    fn fragment_only(
        mut self,
        base_url: &Url,
        mut input: Input<'_>,
    ) -> ParseResult<Url> {}
    pub fn parse_fragment(&mut self, input: Input<'_>) {}
    #[inline]
    fn check_url_code_point(&self, c: char, input: &Input<'_>) {}
}
#[inline]
pub fn to_u32(i: usize) -> ParseResult<u32> {
    if i <= u32::MAX as usize { Ok(i as u32) } else { Err(ParseError::Overflow) }
}
