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
#[derive(Copy, Clone)]
#[must_use]
pub struct ParseOptions<'a> {
    base_url: Option<&'a Url>,
    encoding_override: EncodingOverride<'a>,
    violation_fn: Option<&'a dyn Fn(SyntaxViolation)>,
}
pub struct Parser<'a> {
    pub serialization: String,
    pub base_url: Option<&'a Url>,
    pub query_encoding_override: EncodingOverride<'a>,
    pub violation_fn: Option<&'a dyn Fn(SyntaxViolation)>,
    pub context: Context,
}
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
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Context {
    UrlParser,
    Setter,
    PathSegmentSetter,
}
impl<'a> ParseOptions<'a> {
    pub fn base_url(mut self, new: Option<&'a Url>) -> Self {
        self.base_url = new;
        self
    }
    pub fn encoding_override(mut self, new: EncodingOverride<'a>) -> Self {
        self.encoding_override = new;
        self
    }
    pub fn syntax_violation_callback(
        mut self,
        new: Option<&'a dyn Fn(SyntaxViolation)>,
    ) -> Self {
        self.violation_fn = new;
        self
    }
    pub fn parse(self, input: &str) -> Result<Url, crate::ParseError> {
        Parser {
            serialization: String::with_capacity(input.len()),
            base_url: self.base_url,
            query_encoding_override: self.encoding_override,
            violation_fn: self.violation_fn,
            context: Context::UrlParser,
        }
            .parse_url(input)
    }
}
impl<'a> Parser<'a> {
    fn log_violation(&self, v: SyntaxViolation) {}
    fn log_violation_if(&self, v: SyntaxViolation, test: impl FnOnce() -> bool) {}
    pub fn for_setter(serialization: String) -> Parser<'a> {}
    pub fn parse_url(mut self, input: &str) -> ParseResult<Url> {
        let input = Input::new_trim_c0_control_and_space(input, self.violation_fn);
        if let Ok(remaining) = self.parse_scheme(input.clone()) {
            return self.parse_with_scheme(remaining);
        }
        if let Some(base_url) = self.base_url {
            if input.starts_with('#') {
                self.fragment_only(base_url, input)
            } else if base_url.cannot_be_a_base() {
                Err(ParseError::RelativeUrlWithCannotBeABaseBase)
            } else {
                let scheme_type = SchemeType::from(base_url.scheme());
                if scheme_type.is_file() {
                    self.parse_file(input, scheme_type, Some(base_url))
                } else {
                    self.parse_relative(input, scheme_type, base_url)
                }
            }
        } else {
            Err(ParseError::RelativeUrlWithoutBase)
        }
    }
    pub fn parse_scheme<'i>(&mut self, mut input: Input<'i>) -> Result<Input<'i>, ()> {}
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
