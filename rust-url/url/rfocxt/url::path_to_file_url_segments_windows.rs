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
impl Host<String> {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        if input.starts_with('[') {
            if !input.ends_with(']') {
                return Err(ParseError::InvalidIpv6Address);
            }
            return parse_ipv6addr(&input[1..input.len() - 1]).map(Host::Ipv6);
        }
        let domain: Cow<'_, [u8]> = percent_decode(input.as_bytes()).into();
        let domain = Self::domain_to_ascii(&domain)?;
        if domain.is_empty() {
            return Err(ParseError::EmptyHost);
        }
        if ends_in_a_number(&domain) {
            let address = parse_ipv4addr(&domain)?;
            Ok(Host::Ipv4(address))
        } else {
            Ok(Host::Domain(domain.to_string()))
        }
    }
    pub fn parse_opaque(input: &str) -> Result<Self, ParseError> {
        if input.starts_with('[') {
            if !input.ends_with(']') {
                return Err(ParseError::InvalidIpv6Address);
            }
            return parse_ipv6addr(&input[1..input.len() - 1]).map(Host::Ipv6);
        }
        let is_invalid_host_char = |c| {
            matches!(
                c, '\0' | '\t' | '\n' | '\r' | ' ' | '#' | '/' | ':' | '<' | '>' | '?' |
                '@' | '[' | '\\' | ']' | '^' | '|'
            )
        };
        if input.find(is_invalid_host_char).is_some() {
            Err(ParseError::InvalidDomainCharacter)
        } else {
            Ok(Host::Domain(utf8_percent_encode(input, CONTROLS).to_string()))
        }
    }
    fn domain_to_ascii(domain: &[u8]) -> Result<Cow<'_, str>, ParseError> {}
}
#[cfg(feature = "std")]
#[cfg_attr(not(windows), allow(dead_code))]
fn path_to_file_url_segments_windows(
    path: &Path,
    serialization: &mut String,
) -> Result<(u32, HostInternal), ()> {
    use crate::parser::PATH_SEGMENT;
    use percent_encoding::percent_encode;
    use std::path::{Component, Prefix};
    if !path.is_absolute() {
        return Err(());
    }
    let mut components = path.components();
    let host_start = serialization.len() + 1;
    let host_end;
    let host_internal;
    match components.next() {
        Some(Component::Prefix(ref p)) => {
            match p.kind() {
                Prefix::Disk(letter) | Prefix::VerbatimDisk(letter) => {
                    host_end = to_u32(serialization.len()).unwrap();
                    host_internal = HostInternal::None;
                    serialization.push('/');
                    serialization.push(letter as char);
                    serialization.push(':');
                }
                Prefix::UNC(server, share) | Prefix::VerbatimUNC(server, share) => {
                    let host = Host::parse(server.to_str().ok_or(())?).map_err(|_| ())?;
                    write!(serialization, "{}", host).unwrap();
                    host_end = to_u32(serialization.len()).unwrap();
                    host_internal = host.into();
                    serialization.push('/');
                    let share = share.to_str().ok_or(())?;
                    serialization.extend(percent_encode(share.as_bytes(), PATH_SEGMENT));
                }
                _ => return Err(()),
            }
        }
        _ => return Err(()),
    }
    let mut path_only_has_prefix = true;
    for component in components {
        if component == Component::RootDir {
            continue;
        }
        path_only_has_prefix = false;
        let component = component.as_os_str().to_str().ok_or(())?;
        serialization.push('/');
        serialization.extend(percent_encode(component.as_bytes(), PATH_SEGMENT));
    }
    if serialization.len() > host_start
        && parser::is_windows_drive_letter(&serialization[host_start..])
        && path_only_has_prefix
    {
        serialization.push('/');
    }
    Ok((host_end, host_internal))
}
#[inline]
pub fn is_windows_drive_letter(segment: &str) -> bool {
    segment.len() == 2 && starts_with_windows_drive_letter(segment)
}
#[inline]
pub fn to_u32(i: usize) -> ParseResult<u32> {
    if i <= u32::MAX as usize { Ok(i as u32) } else { Err(ParseError::Overflow) }
}
