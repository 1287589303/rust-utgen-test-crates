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
    pub fn parse(self, input: &str) -> Result<Url, crate::ParseError> {}
}
