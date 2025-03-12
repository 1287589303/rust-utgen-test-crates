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
#[derive(Default)]
#[deprecated]
pub struct Idna {
    config: Config,
}
#[derive(Default, Debug)]
#[non_exhaustive]
pub struct Errors {}
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
    pub fn to_unicode(self, domain: &str) -> (String, Result<(), Errors>) {
        let mut codec = Idna::new(self);
        let mut out = String::with_capacity(domain.len());
        let result = codec.to_unicode(domain, &mut out);
        (out, result)
    }
}
impl Idna {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn to_ascii(&mut self, domain: &str, out: &mut String) -> Result<(), Errors> {}
    #[allow(clippy::wrong_self_convention)]
    pub fn to_unicode(&mut self, domain: &str, out: &mut String) -> Result<(), Errors> {
        let mapped = map_transitional(domain, self.config.transitional_processing);
        match Uts46::new()
            .process(
                mapped.as_bytes(),
                self.config.deny_list(),
                self.config.hyphens(),
                ErrorPolicy::MarkErrors,
                |_, _, _| true,
                out,
                None,
            )
        {
            Ok(ProcessingSuccess::Passthrough) => {
                out.push_str(&mapped);
                Ok(())
            }
            Ok(ProcessingSuccess::WroteToSink) => Ok(()),
            Err(ProcessingError::ValidityError) => Err(crate::Errors::default()),
            Err(ProcessingError::SinkError) => unreachable!(),
        }
    }
}
