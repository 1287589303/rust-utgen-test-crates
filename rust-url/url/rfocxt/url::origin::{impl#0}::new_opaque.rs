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
    pub fn new_opaque() -> Origin {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Origin::Opaque(OpaqueOrigin(COUNTER.fetch_add(1, Ordering::SeqCst)))
    }
    pub fn is_tuple(&self) -> bool {}
    pub fn ascii_serialization(&self) -> String {}
    pub fn unicode_serialization(&self) -> String {}
}
