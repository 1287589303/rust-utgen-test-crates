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
    pub fn to_ascii(self, domain: &str) -> Result<String, Errors> {
        let mut result = String::with_capacity(domain.len());
        let mut codec = Idna::new(self);
        codec.to_ascii(domain, &mut result).map(|()| result)
    }
    pub fn to_unicode(self, domain: &str) -> (String, Result<(), Errors>) {}
}
impl Idna {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn to_ascii(&mut self, domain: &str, out: &mut String) -> Result<(), Errors> {
        let mapped = map_transitional(domain, self.config.transitional_processing);
        match Uts46::new()
            .process(
                mapped.as_bytes(),
                self.config.deny_list(),
                self.config.hyphens(),
                ErrorPolicy::FailFast,
                |_, _, _| false,
                out,
                None,
            )
        {
            Ok(ProcessingSuccess::Passthrough) => {
                if self.config.verify_dns_length && !verify_dns_length(&mapped, true) {
                    return Err(crate::Errors::default());
                }
                out.push_str(&mapped);
                Ok(())
            }
            Ok(ProcessingSuccess::WroteToSink) => {
                if self.config.verify_dns_length && !verify_dns_length(out, true) {
                    return Err(crate::Errors::default());
                }
                Ok(())
            }
            Err(ProcessingError::ValidityError) => Err(crate::Errors::default()),
            Err(ProcessingError::SinkError) => unreachable!(),
        }
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn to_unicode(&mut self, domain: &str, out: &mut String) -> Result<(), Errors> {}
}
