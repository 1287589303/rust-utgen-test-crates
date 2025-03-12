use alloc::borrow::Cow;
use alloc::string::String;
pub use uts46::AsciiDenyList;
use uts46::Uts46;
#[allow(deprecated)]
pub use crate::deprecated::{Config, Idna};
#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct AsciiDenyList {
    bits: u128,
}
#[derive(Default, Debug)]
#[non_exhaustive]
pub struct Errors {}
pub fn domain_to_ascii(domain: &str) -> Result<String, Errors> {
    domain_to_ascii_cow(domain.as_bytes(), AsciiDenyList::EMPTY)
        .map(|cow| cow.into_owned())
}
pub fn domain_to_ascii_cow(
    domain: &[u8],
    ascii_deny_list: AsciiDenyList,
) -> Result<Cow<'_, str>, Errors> {
    Uts46::new()
        .to_ascii(
            domain,
            ascii_deny_list,
            uts46::Hyphens::Allow,
            uts46::DnsLength::Ignore,
        )
}
