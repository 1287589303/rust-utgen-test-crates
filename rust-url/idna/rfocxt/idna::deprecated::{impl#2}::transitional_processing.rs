use alloc::borrow::Cow;
use alloc::string::String;
use crate::uts46::*;
use crate::Errors;
#[derive(Clone, Copy)]
#[must_use]
#[deprecated]
pub struct Config {
    use_std3_ascii_rules: bool,
    transitional_processing: bool,
    verify_dns_length: bool,
    check_hyphens: bool,
}
impl Config {
    #[inline]
    pub fn use_std3_ascii_rules(mut self, value: bool) -> Self {
        self.use_std3_ascii_rules = value;
        self
    }
    #[inline]
    pub fn transitional_processing(mut self, value: bool) -> Self {
        self.transitional_processing = value;
        self
    }
    #[inline]
    pub fn verify_dns_length(mut self, value: bool) -> Self {
        self.verify_dns_length = value;
        self
    }
    #[inline]
    pub fn check_hyphens(mut self, value: bool) -> Self {
        self.check_hyphens = value;
        self
    }
    #[inline]
    #[allow(unused_mut)]
    pub fn use_idna_2008_rules(mut self, value: bool) -> Self {
        assert!(! value, "IDNA 2008 rules are no longer supported");
        self
    }
    fn deny_list(&self) -> AsciiDenyList {}
    fn hyphens(&self) -> Hyphens {}
    pub fn to_ascii(self, domain: &str) -> Result<String, Errors> {}
    pub fn to_unicode(self, domain: &str) -> (String, Result<(), Errors>) {}
}
