use crate::parser::{default_port, Context, Input, Parser, SchemeType};
use crate::{Host, ParseError, Position, Url};
use alloc::string::String;
use alloc::string::ToString;
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
pub fn domain_to_ascii(domain: &str) -> String {
    match Host::parse(domain) {
        Ok(Host::Domain(domain)) => domain,
        _ => String::new(),
    }
}
