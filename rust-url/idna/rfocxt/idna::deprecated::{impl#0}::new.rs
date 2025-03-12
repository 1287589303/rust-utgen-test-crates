use alloc::borrow::Cow;
use alloc::string::String;
use crate::uts46::*;
use crate::Errors;
#[derive(Default)]
#[deprecated]
pub struct Idna {
    config: Config,
}
#[derive(Clone, Copy)]
#[must_use]
#[deprecated]
pub struct Config {
    use_std3_ascii_rules: bool,
    transitional_processing: bool,
    verify_dns_length: bool,
    check_hyphens: bool,
}
impl Idna {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn to_ascii(&mut self, domain: &str, out: &mut String) -> Result<(), Errors> {}
    #[allow(clippy::wrong_self_convention)]
    pub fn to_unicode(&mut self, domain: &str, out: &mut String) -> Result<(), Errors> {}
}
