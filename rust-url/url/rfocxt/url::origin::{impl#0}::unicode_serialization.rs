use crate::host::Host;
use crate::parser::default_port;
use crate::Url;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use core::sync::atomic::{AtomicUsize, Ordering};
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct OpaqueOrigin(usize);
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Origin {
    /// A globally unique identifier
    Opaque(OpaqueOrigin),
    /// Consists of the URL's scheme, host and port
    Tuple(String, Host<String>, u16),
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
impl Origin {
    pub fn new_opaque() -> Origin {}
    pub fn is_tuple(&self) -> bool {}
    pub fn ascii_serialization(&self) -> String {}
    pub fn unicode_serialization(&self) -> String {
        match *self {
            Origin::Opaque(_) => "null".to_owned(),
            Origin::Tuple(ref scheme, ref host, port) => {
                let host = match *host {
                    Host::Domain(ref domain) => {
                        let (domain, _errors) = idna::domain_to_unicode(domain);
                        Host::Domain(domain)
                    }
                    _ => host.clone(),
                };
                if default_port(scheme) == Some(port) {
                    format!("{}://{}", scheme, host)
                } else {
                    format!("{}://{}:{}", scheme, host, port)
                }
            }
        }
    }
}
pub fn default_port(scheme: &str) -> Option<u16> {
    match scheme {
        "http" | "ws" => Some(80),
        "https" | "wss" => Some(443),
        "ftp" => Some(21),
        _ => None,
    }
}
