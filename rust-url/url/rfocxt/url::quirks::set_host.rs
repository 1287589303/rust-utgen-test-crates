use crate::parser::{default_port, Context, Input, Parser, SchemeType};
use crate::{Host, ParseError, Position, Url};
use alloc::string::String;
use alloc::string::ToString;
#[derive(Clone, Debug)]
pub struct Input<'i> {
    chars: str::Chars<'i>,
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
pub struct Parser<'a> {
    pub serialization: String,
    pub base_url: Option<&'a Url>,
    pub query_encoding_override: EncodingOverride<'a>,
    pub violation_fn: Option<&'a dyn Fn(SyntaxViolation)>,
    pub context: Context,
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
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Context {
    UrlParser,
    Setter,
    PathSegmentSetter,
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SchemeType {
    File,
    SpecialNotFile,
    NotSpecial,
}
impl<S, T> PartialEq<Host<T>> for Host<S>
where
    S: PartialEq<T>,
{
    fn eq(&self, other: &Host<T>) -> bool {
        match (self, other) {
            (Host::Domain(a), Host::Domain(b)) => a == b,
            (Host::Ipv4(a), Host::Ipv4(b)) => a == b,
            (Host::Ipv6(a), Host::Ipv6(b)) => a == b,
            (_, _) => false,
        }
    }
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
    pub fn set_port(&mut self, mut port: Option<u16>) -> Result<(), ()> {}
    fn set_port_internal(&mut self, port: Option<u16>) {}
    pub fn set_host(&mut self, host: Option<&str>) -> Result<(), ParseError> {}
    fn set_host_internal(
        &mut self,
        host: Host<String>,
        opt_new_port: Option<Option<u16>>,
    ) {
        let old_suffix_pos = if opt_new_port.is_some() {
            self.path_start
        } else {
            self.host_end
        };
        let suffix = self.slice(old_suffix_pos..).to_owned();
        self.serialization.truncate(self.host_start as usize);
        if !self.has_authority() {
            debug_assert!(self.slice(self.scheme_end..self.host_start) == ":");
            debug_assert!(self.username_end == self.host_start);
            self.serialization.push('/');
            self.serialization.push('/');
            self.username_end += 2;
            self.host_start += 2;
        }
        write!(& mut self.serialization, "{}", host).unwrap();
        self.host_end = to_u32(self.serialization.len()).unwrap();
        self.host = host.into();
        if let Some(new_port) = opt_new_port {
            self.port = new_port;
            if let Some(port) = new_port {
                write!(& mut self.serialization, ":{}", port).unwrap();
            }
        }
        let new_suffix_pos = to_u32(self.serialization.len()).unwrap();
        self.serialization.push_str(&suffix);
        let adjust = |index: &mut u32| {
            *index -= old_suffix_pos;
            *index += new_suffix_pos;
        };
        adjust(&mut self.path_start);
        if let Some(ref mut index) = self.query_start {
            adjust(index)
        }
        if let Some(ref mut index) = self.fragment_start {
            adjust(index)
        }
    }
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
impl<'a> Parser<'a> {
    fn log_violation(&self, v: SyntaxViolation) {}
    fn log_violation_if(&self, v: SyntaxViolation, test: impl FnOnce() -> bool) {}
    pub fn for_setter(serialization: String) -> Parser<'a> {}
    pub fn parse_url(mut self, input: &str) -> ParseResult<Url> {}
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
    ) -> ParseResult<(Host<String>, Input<'_>)> {
        if scheme_type.is_file() {
            return Parser::get_file_host(input);
        }
        let input_str = input.chars.as_str();
        let mut inside_square_brackets = false;
        let mut has_ignored_chars = false;
        let mut non_ignored_chars = 0;
        let mut bytes = 0;
        for c in input_str.chars() {
            match c {
                ':' if !inside_square_brackets => break,
                '\\' if scheme_type.is_special() => break,
                '/' | '?' | '#' => break,
                ascii_tab_or_new_line_pattern!() => {
                    has_ignored_chars = true;
                }
                '[' => {
                    inside_square_brackets = true;
                    non_ignored_chars += 1;
                }
                ']' => {
                    inside_square_brackets = false;
                    non_ignored_chars += 1;
                }
                _ => non_ignored_chars += 1,
            }
            bytes += c.len_utf8();
        }
        let replaced: String;
        let host_str;
        {
            let host_input = input.by_ref().take(non_ignored_chars);
            if has_ignored_chars {
                replaced = host_input.collect();
                host_str = &*replaced;
            } else {
                for _ in host_input {}
                host_str = &input_str[..bytes];
            }
        }
        if scheme_type == SchemeType::SpecialNotFile && host_str.is_empty() {
            return Err(ParseError::EmptyHost);
        }
        if !scheme_type.is_special() {
            let host = Host::parse_opaque(host_str)?;
            return Ok((host, input));
        }
        let host = Host::parse(host_str)?;
        Ok((host, input))
    }
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
#[allow(clippy::result_unit_err)]
pub fn set_host(url: &mut Url, new_host: &str) -> Result<(), ()> {
    if url.cannot_be_a_base() {
        return Err(());
    }
    let input = Input::new_no_trim(new_host);
    let host;
    let opt_port;
    {
        let scheme = url.scheme();
        let scheme_type = SchemeType::from(scheme);
        if scheme_type == SchemeType::File && new_host.is_empty() {
            url.set_host_internal(Host::Domain(String::new()), None);
            return Ok(());
        }
        if let Ok((h, remaining)) = Parser::parse_host(input, scheme_type) {
            host = h;
            opt_port = if let Some(remaining) = remaining.split_prefix(':') {
                if remaining.is_empty() {
                    None
                } else {
                    Parser::parse_port(
                            remaining,
                            || default_port(scheme),
                            Context::Setter,
                        )
                        .ok()
                        .map(|(port, _remaining)| port)
                }
            } else {
                None
            };
        } else {
            return Err(());
        }
    }
    if host == Host::Domain("".to_string())
        && (!username(url).is_empty() || matches!(opt_port, Some(Some(_)))
            || url.port().is_some())
    {
        return Err(());
    }
    url.set_host_internal(host, opt_port);
    Ok(())
}
#[inline]
pub fn username(url: &Url) -> &str {
    url.username()
}
