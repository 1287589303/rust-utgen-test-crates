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
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Ord, PartialOrd, Hash)]
pub enum Host<S = String> {
    /// A DNS domain name, as '.' dot-separated labels.
    /// Non-ASCII labels are encoded in punycode per IDNA if this is the host of
    /// a special URL, or percent encoded for non-special URLs. Hosts for
    /// non-special URLs are also called opaque hosts.
    Domain(S),
    /// An IPv4 address.
    /// `Url::host_str` returns the serialization of this address,
    /// as four decimal integers separated by `.` dots.
    Ipv4(Ipv4Addr),
    /// An IPv6 address.
    /// `Url::host_str` returns the serialization of that address between `[` and `]` brackets,
    /// in the format per [RFC 5952 *A Recommendation
    /// for IPv6 Address Text Representation*](https://tools.ietf.org/html/rfc5952):
    /// lowercase hexadecimal with maximal `::` compression.
    Ipv6(Ipv6Addr),
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
    pub fn make_relative(&self, url: &Url) -> Option<String> {
        if self.cannot_be_a_base() {
            return None;
        }
        if self.scheme() != url.scheme() || self.host() != url.host()
            || self.port() != url.port()
        {
            return None;
        }
        let mut relative = String::new();
        fn extract_path_filename(s: &str) -> (&str, &str) {
            let last_slash_idx = s.rfind('/').unwrap_or(0);
            let (path, filename) = s.split_at(last_slash_idx);
            if filename.is_empty() { (path, "") } else { (path, &filename[1..]) }
        }
        let (base_path, base_filename) = extract_path_filename(self.path());
        let (url_path, url_filename) = extract_path_filename(url.path());
        let mut base_path = base_path.split('/').peekable();
        let mut url_path = url_path.split('/').peekable();
        while base_path.peek().is_some() && base_path.peek() == url_path.peek() {
            base_path.next();
            url_path.next();
        }
        for base_path_segment in base_path {
            if base_path_segment.is_empty() {
                break;
            }
            if !relative.is_empty() {
                relative.push('/');
            }
            relative.push_str("..");
        }
        for url_path_segment in url_path {
            if !relative.is_empty() {
                relative.push('/');
            }
            relative.push_str(url_path_segment);
        }
        if !relative.is_empty() || base_filename != url_filename {
            if url_filename.is_empty() {
                relative.push('/');
            } else {
                if !relative.is_empty() {
                    relative.push('/');
                }
                relative.push_str(url_filename);
            }
        }
        if let Some(query) = url.query() {
            relative.push('?');
            relative.push_str(query);
        }
        if let Some(fragment) = url.fragment() {
            relative.push('#');
            relative.push_str(fragment);
        }
        Some(relative)
    }
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
    pub fn has_authority(&self) -> bool {}
    pub fn authority(&self) -> &str {}
    #[inline]
    pub fn cannot_be_a_base(&self) -> bool {
        !self.slice(self.scheme_end + 1..).starts_with('/')
    }
    pub fn username(&self) -> &str {}
    pub fn password(&self) -> Option<&str> {}
    pub fn has_host(&self) -> bool {}
    pub fn host_str(&self) -> Option<&str> {}
    pub fn host(&self) -> Option<Host<&str>> {
        match self.host {
            HostInternal::None => None,
            HostInternal::Domain => {
                Some(Host::Domain(self.slice(self.host_start..self.host_end)))
            }
            HostInternal::Ipv4(address) => Some(Host::Ipv4(address)),
            HostInternal::Ipv6(address) => Some(Host::Ipv6(address)),
        }
    }
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
    pub fn path(&self) -> &str {
        match (self.query_start, self.fragment_start) {
            (None, None) => self.slice(self.path_start..),
            (Some(next_component_start), _) | (None, Some(next_component_start)) => {
                self.slice(self.path_start..next_component_start)
            }
        }
    }
    pub fn path_segments(&self) -> Option<str::Split<'_, char>> {}
    pub fn query(&self) -> Option<&str> {
        match (self.query_start, self.fragment_start) {
            (None, _) => None,
            (Some(query_start), None) => {
                debug_assert!(self.byte_at(query_start) == b'?');
                Some(self.slice(query_start + 1..))
            }
            (Some(query_start), Some(fragment_start)) => {
                debug_assert!(self.byte_at(query_start) == b'?');
                Some(self.slice(query_start + 1..fragment_start))
            }
        }
    }
    #[inline]
    pub fn query_pairs(&self) -> form_urlencoded::Parse<'_> {}
    pub fn fragment(&self) -> Option<&str> {
        self.fragment_start
            .map(|start| {
                debug_assert!(self.byte_at(start) == b'#');
                self.slice(start + 1..)
            })
    }
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
    pub fn set_port(&mut self, mut port: Option<u16>) -> Result<(), ()> {}
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
    pub fn set_scheme(&mut self, scheme: &str) -> Result<(), ()> {}
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
