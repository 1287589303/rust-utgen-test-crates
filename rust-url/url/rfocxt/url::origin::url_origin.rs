use crate::host::Host;
use crate::parser::default_port;
use crate::Url;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use core::sync::atomic::{AtomicUsize, Ordering};
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
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Origin {
    /// A globally unique identifier
    Opaque(OpaqueOrigin),
    /// Consists of the URL's scheme, host and port
    Tuple(String, Host<String>, u16),
}
impl Host<&str> {
    pub fn to_owned(&self) -> Host<String> {
        match *self {
            Host::Domain(domain) => Host::Domain(domain.to_owned()),
            Host::Ipv4(address) => Host::Ipv4(address),
            Host::Ipv6(address) => Host::Ipv6(address),
        }
    }
}
impl Origin {
    pub fn new_opaque() -> Origin {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Origin::Opaque(OpaqueOrigin(COUNTER.fetch_add(1, Ordering::SeqCst)))
    }
    pub fn is_tuple(&self) -> bool {}
    pub fn ascii_serialization(&self) -> String {}
    pub fn unicode_serialization(&self) -> String {}
}
impl Url {
    #[inline]
    pub fn parse(input: &str) -> Result<Url, crate::ParseError> {
        Url::options().parse(input)
    }
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
    pub fn cannot_be_a_base(&self) -> bool {}
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
    pub fn port(&self) -> Option<u16> {}
    #[inline]
    pub fn port_or_known_default(&self) -> Option<u16> {
        self.port.or_else(|| parser::default_port(self.scheme()))
    }
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
pub fn url_origin(url: &Url) -> Origin {
    let scheme = url.scheme();
    match scheme {
        "blob" => {
            let result = Url::parse(url.path());
            match result {
                Ok(ref url) => url_origin(url),
                Err(_) => Origin::new_opaque(),
            }
        }
        "ftp" | "http" | "https" | "ws" | "wss" => {
            Origin::Tuple(
                scheme.to_owned(),
                url.host().unwrap().to_owned(),
                url.port_or_known_default().unwrap(),
            )
        }
        "file" => Origin::new_opaque(),
        _ => Origin::new_opaque(),
    }
}
