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
#[derive(Debug, Clone, Copy)]
enum AlreadyAsciiLabel<'a> {
    MixedCaseAscii(&'a [u8]),
    MixedCasePunycode(&'a [u8]),
    Other,
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
    ) -> (usize, bool, bool) {
        let mut iter = domain_name.iter();
        let mut most_recent_label_start = iter.clone();
        loop {
            if let Some(&b) = iter.next() {
                if in_inclusive_range8(b, b'a', b'z') {
                    continue;
                }
                if b == b'.' {
                    most_recent_label_start = iter.clone();
                    continue;
                }
                return self
                    .process_innermost(
                        domain_name,
                        ascii_deny_list,
                        hyphens,
                        fail_fast,
                        domain_buffer,
                        already_punycode,
                        most_recent_label_start.as_slice(),
                    );
            } else {
                return (domain_name.len(), false, false);
            }
        }
    }
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
    ) -> (usize, bool, bool) {
        let deny_list = ascii_deny_list.bits;
        let deny_list_deny_dot = deny_list | DOT_MASK;
        let mut had_errors = false;
        let mut passthrough_up_to = domain_name.len() - tail.len();
        let mut current_label_start;
        let mut seen_label = false;
        let mut in_prefix = true;
        for label in tail.split(|b| *b == b'.') {
            if in_prefix && is_passthrough_ascii_label(label) {
                if seen_label {
                    debug_assert_eq!(domain_name[passthrough_up_to], b'.');
                    passthrough_up_to += 1;
                }
                seen_label = true;
                passthrough_up_to += label.len();
                continue;
            }
            if seen_label {
                if in_prefix {
                    debug_assert_eq!(domain_name[passthrough_up_to], b'.');
                    passthrough_up_to += 1;
                } else {
                    domain_buffer.push('.');
                }
            }
            seen_label = true;
            in_prefix = false;
            current_label_start = domain_buffer.len();
            if !label.is_empty() {
                let (ascii, non_ascii) = split_ascii_fast_path_prefix(label);
                let non_punycode_ascii_label = if non_ascii.is_empty() {
                    if has_punycode_prefix(ascii) {
                        if (ascii.last() != Some(&b'-'))
                            && (ascii.len() - 4 <= PUNYCODE_DECODE_MAX_INPUT_LENGTH)
                        {
                            if let Ok(decode) = Decoder::default()
                                .decode::<u8, InternalCaller>(&ascii[4..])
                            {
                                let mut label_buffer = SmallVec::<[char; 59]>::new();
                                label_buffer.extend(decode);
                                if self
                                    .after_punycode_decode(
                                        domain_buffer,
                                        current_label_start,
                                        &label_buffer,
                                        deny_list_deny_dot,
                                        fail_fast,
                                        &mut had_errors,
                                    )
                                {
                                    return (0, false, true);
                                }
                                if self
                                    .check_label(
                                        hyphens,
                                        &mut domain_buffer[current_label_start..],
                                        fail_fast,
                                        &mut had_errors,
                                        true,
                                        true,
                                    )
                                {
                                    return (0, false, true);
                                }
                            } else {
                                if fail_fast {
                                    return (0, false, true);
                                }
                                had_errors = true;
                                domain_buffer.push('\u{FFFD}');
                                let mut iter = ascii.iter();
                                let _ = iter.next();
                                domain_buffer
                                    .extend(
                                        iter
                                            .map(|c| {
                                                apply_ascii_deny_list_to_potentially_upper_case_ascii(
                                                    *c,
                                                    deny_list,
                                                )
                                            }),
                                    );
                            };
                            already_punycode
                                .push(AlreadyAsciiLabel::MixedCasePunycode(label));
                            continue;
                        } else if fail_fast {
                            return (0, false, true);
                        }
                        false
                    } else {
                        true
                    }
                } else {
                    false
                };
                for c in ascii
                    .iter()
                    .map(|c| {
                        apply_ascii_deny_list_to_potentially_upper_case_ascii(
                            *c,
                            deny_list,
                        )
                    })
                {
                    if c == '\u{FFFD}' {
                        if fail_fast {
                            return (0, false, true);
                        }
                        had_errors = true;
                    }
                    domain_buffer.push(c);
                }
                if non_punycode_ascii_label {
                    if hyphens != Hyphens::Allow
                        && check_hyphens(
                            &mut domain_buffer[current_label_start..],
                            hyphens == Hyphens::CheckFirstLast,
                            fail_fast,
                            &mut had_errors,
                        )
                    {
                        return (0, false, true);
                    }
                    already_punycode
                        .push(
                            if had_errors {
                                AlreadyAsciiLabel::Other
                            } else {
                                AlreadyAsciiLabel::MixedCaseAscii(label)
                            },
                        );
                    continue;
                }
                already_punycode.push(AlreadyAsciiLabel::Other);
                let mut first_needs_combining_mark_check = ascii.is_empty();
                let mut needs_contextj_check = !non_ascii.is_empty();
                let mut mapping = self
                    .data
                    .map_normalize(non_ascii.chars())
                    .map(|c| apply_ascii_deny_list_to_lower_cased_unicode(c, deny_list));
                loop {
                    let n = mapping.next();
                    match n {
                        None | Some('.') => {
                            if domain_buffer[current_label_start..]
                                .starts_with(&['x', 'n', '-', '-'])
                            {
                                let mut punycode_precondition_failed = false;
                                for c in domain_buffer[current_label_start + 4..].iter_mut()
                                {
                                    if !c.is_ascii() {
                                        if fail_fast {
                                            return (0, false, true);
                                        }
                                        had_errors = true;
                                        *c = '\u{FFFD}';
                                        punycode_precondition_failed = true;
                                    }
                                }
                                if let Some(last) = domain_buffer.last_mut() {
                                    if *last == '-' {
                                        if fail_fast {
                                            return (0, false, true);
                                        }
                                        had_errors = true;
                                        *last = '\u{FFFD}';
                                        punycode_precondition_failed = true;
                                    }
                                } else {
                                    unreachable!();
                                }
                                if domain_buffer.len() - current_label_start - 4
                                    > PUNYCODE_DECODE_MAX_INPUT_LENGTH
                                {
                                    if fail_fast {
                                        return (0, false, true);
                                    }
                                    had_errors = true;
                                    domain_buffer[current_label_start + 4
                                        + PUNYCODE_DECODE_MAX_INPUT_LENGTH] = '\u{FFFD}';
                                    punycode_precondition_failed = true;
                                }
                                if !punycode_precondition_failed {
                                    if let Ok(decode) = Decoder::default()
                                        .decode::<
                                            char,
                                            InternalCaller,
                                        >(&domain_buffer[current_label_start + 4..])
                                    {
                                        first_needs_combining_mark_check = true;
                                        needs_contextj_check = true;
                                        let mut label_buffer = SmallVec::<[char; 59]>::new();
                                        label_buffer.extend(decode);
                                        domain_buffer.truncate(current_label_start);
                                        if self
                                            .after_punycode_decode(
                                                domain_buffer,
                                                current_label_start,
                                                &label_buffer,
                                                deny_list_deny_dot,
                                                fail_fast,
                                                &mut had_errors,
                                            )
                                        {
                                            return (0, false, true);
                                        }
                                    } else {
                                        if fail_fast {
                                            return (0, false, true);
                                        }
                                        had_errors = true;
                                        domain_buffer[current_label_start] = '\u{FFFD}';
                                        needs_contextj_check = false;
                                        first_needs_combining_mark_check = false;
                                    };
                                } else {
                                    first_needs_combining_mark_check = false;
                                    needs_contextj_check = false;
                                }
                            }
                            if self
                                .check_label(
                                    hyphens,
                                    &mut domain_buffer[current_label_start..],
                                    fail_fast,
                                    &mut had_errors,
                                    first_needs_combining_mark_check,
                                    needs_contextj_check,
                                )
                            {
                                return (0, false, true);
                            }
                            if n.is_none() {
                                break;
                            }
                            domain_buffer.push('.');
                            current_label_start = domain_buffer.len();
                            first_needs_combining_mark_check = true;
                            needs_contextj_check = true;
                            already_punycode.push(AlreadyAsciiLabel::Other);
                        }
                        Some(c) => {
                            if c == '\u{FFFD}' {
                                if fail_fast {
                                    return (0, false, true);
                                }
                                had_errors = true;
                            }
                            domain_buffer.push(c);
                        }
                    }
                }
            } else {
                already_punycode.push(AlreadyAsciiLabel::MixedCaseAscii(label));
            }
        }
        let is_bidi = self.is_bidi(domain_buffer);
        if is_bidi {
            for label in domain_buffer.split_mut(|c| *c == '.') {
                if let Some((first, tail)) = label.split_first_mut() {
                    let first_bc = self.data.bidi_class(*first);
                    if !FIRST_BC_MASK.intersects(first_bc.to_mask()) {
                        if fail_fast {
                            return (0, false, true);
                        }
                        had_errors = true;
                        *first = '\u{FFFD}';
                        continue;
                    }
                    let is_ltr = first_bc.is_ltr();
                    let mut middle = tail;
                    #[allow(clippy::while_let_loop)]
                    loop {
                        if let Some((last, prior)) = middle.split_last_mut() {
                            let last_bc = self.data.bidi_class(*last);
                            if last_bc.is_nonspacing_mark() {
                                middle = prior;
                                continue;
                            }
                            let last_mask = if is_ltr {
                                LAST_LTR_MASK
                            } else {
                                LAST_RTL_MASK
                            };
                            if !last_mask.intersects(last_bc.to_mask()) {
                                if fail_fast {
                                    return (0, false, true);
                                }
                                had_errors = true;
                                *last = '\u{FFFD}';
                            }
                            if is_ltr {
                                for c in prior.iter_mut() {
                                    let bc = self.data.bidi_class(*c);
                                    if !MIDDLE_LTR_MASK.intersects(bc.to_mask()) {
                                        if fail_fast {
                                            return (0, false, true);
                                        }
                                        had_errors = true;
                                        *c = '\u{FFFD}';
                                    }
                                }
                            } else {
                                let mut numeral_state = RtlNumeralState::Undecided;
                                for c in prior.iter_mut() {
                                    let bc = self.data.bidi_class(*c);
                                    if !MIDDLE_RTL_MASK.intersects(bc.to_mask()) {
                                        if fail_fast {
                                            return (0, false, true);
                                        }
                                        had_errors = true;
                                        *c = '\u{FFFD}';
                                    } else {
                                        match numeral_state {
                                            RtlNumeralState::Undecided => {
                                                if bc.is_european_number() {
                                                    numeral_state = RtlNumeralState::European;
                                                } else if bc.is_arabic_number() {
                                                    numeral_state = RtlNumeralState::Arabic;
                                                }
                                            }
                                            RtlNumeralState::European => {
                                                if bc.is_arabic_number() {
                                                    if fail_fast {
                                                        return (0, false, true);
                                                    }
                                                    had_errors = true;
                                                    *c = '\u{FFFD}';
                                                }
                                            }
                                            RtlNumeralState::Arabic => {
                                                if bc.is_european_number() {
                                                    if fail_fast {
                                                        return (0, false, true);
                                                    }
                                                    had_errors = true;
                                                    *c = '\u{FFFD}';
                                                }
                                            }
                                        }
                                    }
                                }
                                if (numeral_state == RtlNumeralState::European
                                    && last_bc.is_arabic_number())
                                    || (numeral_state == RtlNumeralState::Arabic
                                        && last_bc.is_european_number())
                                {
                                    if fail_fast {
                                        return (0, false, true);
                                    }
                                    had_errors = true;
                                    *last = '\u{FFFD}';
                                }
                            }
                            break;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        (passthrough_up_to, is_bidi, had_errors)
    }
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
#[inline(always)]
fn in_inclusive_range8(u: u8, start: u8, end: u8) -> bool {
    u.wrapping_sub(start) <= (end - start)
}
