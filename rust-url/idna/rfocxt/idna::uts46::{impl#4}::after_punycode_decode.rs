use crate::punycode::Decoder;
use crate::punycode::InternalCaller;
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt::Write;
use idna_adapter::*;
use smallvec::SmallVec;
use utf8_iter::Utf8CharsEx;
const PUNYCODE_DECODE_MAX_INPUT_LENGTH: usize = 2000;
const PUNYCODE_ENCODE_MAX_INPUT_LENGTH: usize = 1000;
const UPPER_CASE_MASK: u128 = upper_case_mask();
const GLYPHLESS_MASK: u128 = glyphless_mask();
const DOT_MASK: u128 = 1 << b'.';
const PUNYCODE_PREFIX: u32 = ((b'-' as u32) << 24) | ((b'-' as u32) << 16)
    | ((b'N' as u32) << 8) | b'X' as u32;
const PUNYCODE_PREFIX_MASK: u32 = (0xFF << 24) | (0xFF << 16) | (0xDF << 8) | 0xDF;
pub struct Uts46 {
    data: idna_adapter::Adapter,
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
    ) -> Result<Cow<'a, str>, crate::Errors> {}
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
    ) -> bool {
        for c in self
            .data
            .normalize_validate(label_buffer.iter().copied())
            .map(|c| apply_ascii_deny_list_to_lower_cased_unicode(c, deny_list_deny_dot))
        {
            if c == '\u{FFFD}' {
                if fail_fast {
                    return true;
                }
                *had_errors = true;
            }
            domain_buffer.push(c);
        }
        let normalized = &mut domain_buffer[current_label_start..];
        if let Err(()) = normalized
            .iter_mut()
            .zip(label_buffer.iter())
            .try_for_each(|(norm_c, decoded_c)| {
                if *norm_c == *decoded_c {
                    Ok(())
                } else {
                    *norm_c = '\u{FFFD}';
                    Err(())
                }
            })
        {
            if fail_fast {
                return true;
            }
            *had_errors = true;
        }
        false
    }
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
