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
    pub fn host(&self) -> Option<Host<&str>> {}
    pub fn domain(&self) -> Option<&str> {}
    #[inline]
    pub fn port(&self) -> Option<u16> {}
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
    fn take_after_path(&mut self) -> String {
        match (self.query_start, self.fragment_start) {
            (Some(i), _) | (None, Some(i)) => {
                let after_path = self.slice(i..).to_owned();
                self.serialization.truncate(i as usize);
                after_path
            }
            (None, None) => String::new(),
        }
    }
    pub fn set_path(&mut self, mut path: &str) {
        let after_path = self.take_after_path();
        let old_after_path_pos = to_u32(self.serialization.len()).unwrap();
        let cannot_be_a_base = self.cannot_be_a_base();
        let scheme_type = SchemeType::from(self.scheme());
        self.serialization.truncate(self.path_start as usize);
        self.mutate(|parser| {
            if cannot_be_a_base {
                if path.starts_with('/') {
                    parser.serialization.push_str("%2F");
                    path = &path[1..];
                }
                parser.parse_cannot_be_a_base_path(parser::Input::new_no_trim(path));
            } else {
                let mut has_host = true;
                parser
                    .parse_path_start(
                        scheme_type,
                        &mut has_host,
                        parser::Input::new_no_trim(path),
                    );
            }
        });
        self.restore_after_path(old_after_path_pos, &after_path);
    }
    #[allow(clippy::result_unit_err)]
    pub fn path_segments_mut(&mut self) -> Result<PathSegmentsMut<'_>, ()> {}
    fn restore_after_path(&mut self, old_after_path_position: u32, after_path: &str) {
        let new_after_path_position = to_u32(self.serialization.len()).unwrap();
        let adjust = |index: &mut u32| {
            *index -= old_after_path_position;
            *index += new_after_path_position;
        };
        if let Some(ref mut index) = self.query_start {
            adjust(index)
        }
        if let Some(ref mut index) = self.fragment_start {
            adjust(index)
        }
        self.serialization.push_str(after_path)
    }
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
#[inline]
pub fn to_u32(i: usize) -> ParseResult<u32> {
    if i <= u32::MAX as usize { Ok(i as u32) } else { Err(ParseError::Overflow) }
}
