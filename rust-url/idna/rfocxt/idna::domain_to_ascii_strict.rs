use alloc::borrow::Cow;
use alloc::string::String;
pub use uts46::AsciiDenyList;
use uts46::Uts46;
#[allow(deprecated)]
pub use crate::deprecated::{Config, Idna};
pub struct Uts46 {
    data: idna_adapter::Adapter,
}
#[derive(Default, Debug)]
#[non_exhaustive]
pub struct Errors {}
#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct AsciiDenyList {
    bits: u128,
}
#[derive(PartialEq, Eq, Copy, Clone)]
#[non_exhaustive]
pub enum Hyphens {
    /// _CheckHyphens=false_: Do not place positional restrictions on hyphens.
    ///
    /// This mode is used by the WHATWG URL Standard for normal User Agent processing
    /// (i.e. not conformance checking).
    Allow,
    /// Prohibit hyphens in the first and last position in the label but allow in
    /// the third and fourth position.
    ///
    /// Note that this mode rejects real-world names, including some GitHub user pages.
    CheckFirstLast,
    /// _CheckHyphens=true_: Prohibit hyphens in the first, third, fourth,
    /// and last position in the label.
    ///
    /// Note that this mode rejects real-world names, including YouTube CDN nodes
    /// and some GitHub user pages.
    Check,
}
#[derive(PartialEq, Eq, Copy, Clone)]
#[non_exhaustive]
pub enum DnsLength {
    /// _VerifyDNSLength=false_. (Possibly relevant for allowing non-DNS naming systems.)
    Ignore,
    /// _VerifyDNSLength=true_ with the exception that the trailing root label dot is
    /// allowed.
    VerifyAllowRootDot,
    /// _VerifyDNSLength=true_. (The trailing root label dot is not allowed.)
    Verify,
}
impl Uts46 {
    #[cfg(feature = "compiled_data")]
    pub const fn new() -> Self {
        Self {
            data: idna_adapter::Adapter::new(),
        }
    }
    pub fn to_ascii<'a>(
        &self,
        domain_name: &'a [u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
        dns_length: DnsLength,
    ) -> Result<Cow<'a, str>, crate::Errors> {
        let mut s = String::new();
        match self
            .process(
                domain_name,
                ascii_deny_list,
                hyphens,
                ErrorPolicy::FailFast,
                |_, _, _| false,
                &mut s,
                None,
            )
        {
            Ok(ProcessingSuccess::Passthrough) => {
                let cow = Cow::Borrowed(unsafe {
                    core::str::from_utf8_unchecked(domain_name)
                });
                if dns_length != DnsLength::Ignore
                    && !verify_dns_length(
                        &cow,
                        dns_length == DnsLength::VerifyAllowRootDot,
                    )
                {
                    Err(crate::Errors::default())
                } else {
                    Ok(cow)
                }
            }
            Ok(ProcessingSuccess::WroteToSink) => {
                let cow: Cow<'_, str> = Cow::Owned(s);
                if dns_length != DnsLength::Ignore
                    && !verify_dns_length(
                        &cow,
                        dns_length == DnsLength::VerifyAllowRootDot,
                    )
                {
                    Err(crate::Errors::default())
                } else {
                    Ok(cow)
                }
            }
            Err(ProcessingError::ValidityError) => Err(crate::Errors::default()),
            Err(ProcessingError::SinkError) => unreachable!(),
        }
    }
    pub fn to_unicode<'a>(
        &self,
        domain_name: &'a [u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
    ) -> (Cow<'a, str>, Result<(), crate::Errors>) {}
    pub fn to_user_interface<'a, OutputUnicode: FnMut(&[char], &[char], bool) -> bool>(
        &self,
        domain_name: &'a [u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
        output_as_unicode: OutputUnicode,
    ) -> (Cow<'a, str>, Result<(), crate::Errors>) {}
    #[allow(clippy::too_many_arguments)]
    pub fn process<
        W: Write + ?Sized,
        OutputUnicode: FnMut(&[char], &[char], bool) -> bool,
    >(
        &self,
        domain_name: &[u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
        error_policy: ErrorPolicy,
        mut output_as_unicode: OutputUnicode,
        sink: &mut W,
        ascii_sink: Option<&mut W>,
    ) -> Result<ProcessingSuccess, ProcessingError> {}
    #[inline(always)]
    fn process_inner<'a>(
        &self,
        domain_name: &'a [u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
        fail_fast: bool,
        domain_buffer: &mut SmallVec<[char; 253]>,
        already_punycode: &mut SmallVec<[AlreadyAsciiLabel<'a>; 8]>,
    ) -> (usize, bool, bool) {}
    #[allow(clippy::too_many_arguments)]
    #[inline(never)]
    fn process_innermost<'a>(
        &self,
        domain_name: &'a [u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
        fail_fast: bool,
        domain_buffer: &mut SmallVec<[char; 253]>,
        already_punycode: &mut SmallVec<[AlreadyAsciiLabel<'a>; 8]>,
        tail: &'a [u8],
    ) -> (usize, bool, bool) {}
    #[inline(never)]
    fn after_punycode_decode(
        &self,
        domain_buffer: &mut SmallVec<[char; 253]>,
        current_label_start: usize,
        label_buffer: &[char],
        deny_list_deny_dot: u128,
        fail_fast: bool,
        had_errors: &mut bool,
    ) -> bool {}
    #[inline(never)]
    fn check_label(
        &self,
        hyphens: Hyphens,
        mut_label: &mut [char],
        fail_fast: bool,
        had_errors: &mut bool,
        first_needs_combining_mark_check: bool,
        needs_contextj_check: bool,
    ) -> bool {}
    #[inline(always)]
    fn has_appropriately_joining_char<I: Iterator<Item = char>>(
        &self,
        iter: I,
        required_mask: JoiningTypeMask,
    ) -> bool {}
    #[inline(always)]
    fn is_bidi(&self, buffer: &[char]) -> bool {}
}
pub fn domain_to_ascii_strict(domain: &str) -> Result<String, Errors> {
    Uts46::new()
        .to_ascii(
            domain.as_bytes(),
            uts46::AsciiDenyList::STD3,
            uts46::Hyphens::Check,
            uts46::DnsLength::Verify,
        )
        .map(|cow| cow.into_owned())
}
