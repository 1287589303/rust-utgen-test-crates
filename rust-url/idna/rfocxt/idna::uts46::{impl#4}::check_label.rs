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
    ) -> bool {
        if hyphens != Hyphens::Allow
            && check_hyphens(
                mut_label,
                hyphens == Hyphens::CheckFirstLast,
                fail_fast,
                had_errors,
            )
        {
            return true;
        }
        if first_needs_combining_mark_check {
            if let Some(first) = mut_label.first_mut() {
                if self.data.is_mark(*first) {
                    if fail_fast {
                        return true;
                    }
                    *had_errors = true;
                    *first = '\u{FFFD}';
                }
            }
        }
        if needs_contextj_check {
            for i in 0..mut_label.len() {
                let c = mut_label[i];
                if !in_inclusive_range_char(c, '\u{200C}', '\u{200D}') {
                    continue;
                }
                let (head, joiner_and_tail) = mut_label.split_at_mut(i);
                if let Some((joiner, tail)) = joiner_and_tail.split_first_mut() {
                    if let Some(previous) = head.last() {
                        if self.data.is_virama(*previous) {
                            continue;
                        }
                    } else {
                        if fail_fast {
                            return true;
                        }
                        *had_errors = true;
                        *joiner = '\u{FFFD}';
                        continue;
                    }
                    if c == '\u{200D}' {
                        if fail_fast {
                            return true;
                        }
                        *had_errors = true;
                        *joiner = '\u{FFFD}';
                        continue;
                    }
                    debug_assert_eq!(c, '\u{200C}');
                    if !self
                        .has_appropriately_joining_char(
                            head.iter().rev().copied(),
                            LEFT_OR_DUAL_JOINING_MASK,
                        )
                        || !self
                            .has_appropriately_joining_char(
                                tail.iter().copied(),
                                RIGHT_OR_DUAL_JOINING_MASK,
                            )
                    {
                        if fail_fast {
                            return true;
                        }
                        *had_errors = true;
                        *joiner = '\u{FFFD}';
                    }
                } else {
                    debug_assert!(false);
                }
            }
        }
        if !is_ascii(mut_label) && mut_label.len() > PUNYCODE_ENCODE_MAX_INPUT_LENGTH {
            if fail_fast {
                return true;
            }
            *had_errors = true;
            mut_label[PUNYCODE_ENCODE_MAX_INPUT_LENGTH] = '\u{FFFD}';
        }
        false
    }
    #[inline(always)]
    fn has_appropriately_joining_char<I: Iterator<Item = char>>(
        &self,
        iter: I,
        required_mask: JoiningTypeMask,
    ) -> bool {}
    #[inline(always)]
    fn is_bidi(&self, buffer: &[char]) -> bool {}
}
#[inline(always)]
fn is_ascii(label: &[char]) -> bool {
    for c in label.iter() {
        if !c.is_ascii() {
            return false;
        }
    }
    true
}
#[inline(always)]
fn in_inclusive_range_char(c: char, start: char, end: char) -> bool {
    u32::from(c).wrapping_sub(u32::from(start)) <= (u32::from(end) - u32::from(start))
}
fn check_hyphens(
    mut_label: &mut [char],
    allow_third_fourth: bool,
    fail_fast: bool,
    had_errors: &mut bool,
) -> bool {
    if let Some(first) = mut_label.first_mut() {
        if *first == '-' {
            if fail_fast {
                return true;
            }
            *had_errors = true;
            *first = '\u{FFFD}';
        }
    }
    if let Some(last) = mut_label.last_mut() {
        if *last == '-' {
            if fail_fast {
                return true;
            }
            *had_errors = true;
            *last = '\u{FFFD}';
        }
    }
    if allow_third_fourth {
        return false;
    }
    if mut_label.len() >= 4 && mut_label[2] == '-' && mut_label[3] == '-' {
        if fail_fast {
            return true;
        }
        *had_errors = true;
        mut_label[2] = '\u{FFFD}';
        mut_label[3] = '\u{FFFD}';
    }
    false
}
