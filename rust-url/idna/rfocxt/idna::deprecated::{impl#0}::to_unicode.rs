use alloc::borrow::Cow;
use alloc::string::String;
use crate::uts46::*;
use crate::Errors;
#[derive(Default)]
#[deprecated]
pub struct Idna {
    config: Config,
}
pub struct Uts46 {
    data: idna_adapter::Adapter,
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
#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct AsciiDenyList {
    bits: u128,
}
#[derive(Default, Debug)]
#[non_exhaustive]
pub struct Errors {}
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ProcessingSuccess {
    /// There were no errors. The caller must consider the input to be the output.
    ///
    /// This asserts that the input can be safely passed to [`core::str::from_utf8_unchecked`].
    ///
    /// (Distinct from `WroteToSink` in order to allow `Cow` behavior to be implemented on top of
    /// [`Uts46::process`].)
    Passthrough,
    /// There were no errors. The caller must consider what was written to the sink to be the output.
    ///
    /// (Distinct from `Passthrough` in order to allow `Cow` behavior to be implemented on top of
    /// [`Uts46::process`].)
    WroteToSink,
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
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ProcessingError {
    /// There was a validity error according to the chosen options.
    ///
    /// In case of `Operation::ToAscii`, there is no output. Otherwise, output was written to the
    /// sink and the output contains at least one U+FFFD REPLACEMENT CHARACTER to denote an error.
    ValidityError,
    /// The sink emitted [`core::fmt::Error`]. The partial output written to the sink must not
    /// be used.
    SinkError,
}
#[derive(PartialEq, Eq, Copy, Clone)]
#[non_exhaustive]
pub enum ErrorPolicy {
    /// Return as early as possible without producing output in case of error.
    FailFast,
    /// In case of error, mark errors with the REPLACEMENT CHARACTER. (The output
    /// containing REPLACEMENT CHARACTERs may be show to the user to illustrate
    /// what was wrong but must not be used for naming in a network protocol.)
    MarkErrors,
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
    ) -> Result<ProcessingSuccess, ProcessingError> {
        let fail_fast = error_policy == ErrorPolicy::FailFast;
        let mut domain_buffer = SmallVec::<[char; 253]>::new();
        let mut already_punycode = SmallVec::<[AlreadyAsciiLabel; 8]>::new();
        let (passthrough_up_to, is_bidi, had_errors) = self
            .process_inner(
                domain_name,
                ascii_deny_list,
                hyphens,
                fail_fast,
                &mut domain_buffer,
                &mut already_punycode,
            );
        if passthrough_up_to == domain_name.len() {
            debug_assert!(! had_errors);
            return Ok(ProcessingSuccess::Passthrough);
        }
        if fail_fast && had_errors {
            return Err(ProcessingError::ValidityError);
        }
        debug_assert_eq!(had_errors, domain_buffer.contains(&'\u{FFFD}'));
        let without_dot = if let Some(without_dot) = domain_buffer.strip_suffix(&['.']) {
            without_dot
        } else {
            &domain_buffer[..]
        };
        let tld = without_dot.rsplit(|c| *c == '.').next().unwrap();
        let mut had_unicode_output = false;
        let mut seen_label = false;
        let mut already_punycode_iter = already_punycode.iter();
        let mut passthrough_up_to_extended = passthrough_up_to;
        let mut flushed_prefix = false;
        for label in domain_buffer.split(|c| *c == '.') {
            let input_punycode = *already_punycode_iter.next().unwrap();
            if seen_label {
                if flushed_prefix {
                    sink.write_char('.')?;
                } else {
                    debug_assert_eq!(domain_name[passthrough_up_to_extended], b'.');
                    passthrough_up_to_extended += 1;
                    if passthrough_up_to_extended == domain_name.len() {
                        debug_assert!(! had_errors);
                        return Ok(ProcessingSuccess::Passthrough);
                    }
                }
            }
            seen_label = true;
            if let AlreadyAsciiLabel::MixedCaseAscii(mixed_case) = input_punycode {
                if let Some(first_upper_case) = mixed_case
                    .iter()
                    .position(|c| c.is_ascii_uppercase())
                {
                    let (head, tail) = mixed_case.split_at(first_upper_case);
                    let slice_to_write = if flushed_prefix {
                        head
                    } else {
                        flushed_prefix = true;
                        passthrough_up_to_extended += head.len();
                        debug_assert_ne!(passthrough_up_to_extended, domain_name.len());
                        &domain_name[..passthrough_up_to_extended]
                    };
                    sink.write_str(unsafe {
                        core::str::from_utf8_unchecked(slice_to_write)
                    })?;
                    for c in tail.iter() {
                        sink.write_char(char::from(c.to_ascii_lowercase()))?;
                    }
                } else if flushed_prefix {
                    sink.write_str(unsafe {
                        core::str::from_utf8_unchecked(mixed_case)
                    })?;
                } else {
                    passthrough_up_to_extended += mixed_case.len();
                    if passthrough_up_to_extended == domain_name.len() {
                        debug_assert!(! had_errors);
                        return Ok(ProcessingSuccess::Passthrough);
                    }
                }
                continue;
            }
            let potentially_punycode = if fail_fast {
                debug_assert!(
                    classify_for_punycode(label) != PunycodeClassification::Error
                );
                !is_ascii(label)
            } else {
                classify_for_punycode(label) == PunycodeClassification::Unicode
            };
            let passthrough = if potentially_punycode {
                let unicode = output_as_unicode(label, tld, is_bidi);
                had_unicode_output |= unicode;
                unicode
            } else {
                true
            };
            if passthrough {
                if !flushed_prefix {
                    flushed_prefix = true;
                    sink.write_str(unsafe {
                        core::str::from_utf8_unchecked(
                            &domain_name[..passthrough_up_to_extended],
                        )
                    })?;
                }
                for c in label.iter().copied() {
                    sink.write_char(c)?;
                }
            } else if let AlreadyAsciiLabel::MixedCasePunycode(mixed_case) = input_punycode {
                if let Some(first_upper_case) = mixed_case
                    .iter()
                    .position(|c| c.is_ascii_uppercase())
                {
                    let (head, tail) = mixed_case.split_at(first_upper_case);
                    let slice_to_write = if flushed_prefix {
                        head
                    } else {
                        flushed_prefix = true;
                        passthrough_up_to_extended += head.len();
                        debug_assert_ne!(passthrough_up_to_extended, domain_name.len());
                        &domain_name[..passthrough_up_to_extended]
                    };
                    sink.write_str(unsafe {
                        core::str::from_utf8_unchecked(slice_to_write)
                    })?;
                    for c in tail.iter() {
                        sink.write_char(char::from(c.to_ascii_lowercase()))?;
                    }
                } else if flushed_prefix {
                    sink.write_str(unsafe {
                        core::str::from_utf8_unchecked(mixed_case)
                    })?;
                } else {
                    passthrough_up_to_extended += mixed_case.len();
                    if passthrough_up_to_extended == domain_name.len() {
                        debug_assert!(! had_errors);
                        return Ok(ProcessingSuccess::Passthrough);
                    }
                }
            } else {
                if !flushed_prefix {
                    flushed_prefix = true;
                    sink.write_str(unsafe {
                        core::str::from_utf8_unchecked(
                            &domain_name[..passthrough_up_to_extended],
                        )
                    })?;
                }
                write_punycode_label(label, sink)?;
            }
        }
        if had_errors {
            return Err(ProcessingError::ValidityError);
        }
        if had_unicode_output {
            if let Some(sink) = ascii_sink {
                let mut seen_label = false;
                let mut already_punycode_iter = already_punycode.iter();
                let mut passthrough_up_to_extended = passthrough_up_to;
                let mut flushed_prefix = false;
                for label in domain_buffer.split(|c| *c == '.') {
                    let input_punycode = *already_punycode_iter.next().unwrap();
                    if seen_label {
                        if flushed_prefix {
                            sink.write_char('.')?;
                        } else {
                            debug_assert_eq!(
                                domain_name[passthrough_up_to_extended], b'.'
                            );
                            passthrough_up_to_extended += 1;
                        }
                    }
                    seen_label = true;
                    if let AlreadyAsciiLabel::MixedCaseAscii(mixed_case) = input_punycode {
                        if let Some(first_upper_case) = mixed_case
                            .iter()
                            .position(|c| c.is_ascii_uppercase())
                        {
                            let (head, tail) = mixed_case.split_at(first_upper_case);
                            let slice_to_write = if flushed_prefix {
                                head
                            } else {
                                flushed_prefix = true;
                                passthrough_up_to_extended += head.len();
                                debug_assert_ne!(
                                    passthrough_up_to_extended, domain_name.len()
                                );
                                &domain_name[..passthrough_up_to_extended]
                            };
                            sink.write_str(unsafe {
                                core::str::from_utf8_unchecked(slice_to_write)
                            })?;
                            for c in tail.iter() {
                                sink.write_char(char::from(c.to_ascii_lowercase()))?;
                            }
                        } else if flushed_prefix {
                            sink.write_str(unsafe {
                                core::str::from_utf8_unchecked(mixed_case)
                            })?;
                        } else {
                            passthrough_up_to_extended += mixed_case.len();
                        }
                        continue;
                    }
                    if is_ascii(label) {
                        if !flushed_prefix {
                            flushed_prefix = true;
                            sink.write_str(unsafe {
                                core::str::from_utf8_unchecked(
                                    &domain_name[..passthrough_up_to_extended],
                                )
                            })?;
                        }
                        for c in label.iter().copied() {
                            sink.write_char(c)?;
                        }
                    } else if let AlreadyAsciiLabel::MixedCasePunycode(mixed_case) = input_punycode {
                        if let Some(first_upper_case) = mixed_case
                            .iter()
                            .position(|c| c.is_ascii_uppercase())
                        {
                            let (head, tail) = mixed_case.split_at(first_upper_case);
                            let slice_to_write = if flushed_prefix {
                                head
                            } else {
                                flushed_prefix = true;
                                passthrough_up_to_extended += head.len();
                                debug_assert_ne!(
                                    passthrough_up_to_extended, domain_name.len()
                                );
                                &domain_name[..passthrough_up_to_extended]
                            };
                            sink.write_str(unsafe {
                                core::str::from_utf8_unchecked(slice_to_write)
                            })?;
                            for c in tail.iter() {
                                sink.write_char(char::from(c.to_ascii_lowercase()))?;
                            }
                        } else if flushed_prefix {
                            sink.write_str(unsafe {
                                core::str::from_utf8_unchecked(mixed_case)
                            })?;
                        } else {
                            passthrough_up_to_extended += mixed_case.len();
                        }
                    } else {
                        if !flushed_prefix {
                            flushed_prefix = true;
                            sink.write_str(unsafe {
                                core::str::from_utf8_unchecked(
                                    &domain_name[..passthrough_up_to_extended],
                                )
                            })?;
                        }
                        write_punycode_label(label, sink)?;
                    }
                }
                if !flushed_prefix {
                    sink.write_str(unsafe {
                        core::str::from_utf8_unchecked(
                            &domain_name[..passthrough_up_to_extended],
                        )
                    })?;
                }
            }
        }
        Ok(ProcessingSuccess::WroteToSink)
    }
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
    fn deny_list(&self) -> AsciiDenyList {
        if self.use_std3_ascii_rules {
            AsciiDenyList::STD3
        } else {
            AsciiDenyList::EMPTY
        }
    }
    fn hyphens(&self) -> Hyphens {
        if self.check_hyphens { Hyphens::CheckFirstLast } else { Hyphens::Allow }
    }
    pub fn to_ascii(self, domain: &str) -> Result<String, Errors> {}
    pub fn to_unicode(self, domain: &str) -> (String, Result<(), Errors>) {}
}
fn map_transitional(domain: &str, transitional: bool) -> Cow<'_, str> {
    if !transitional {
        return Cow::Borrowed(domain);
    }
    let mut chars = domain.chars();
    loop {
        let prev = chars.clone();
        if let Some(c) = chars.next() {
            match c {
                'ß' | 'ẞ' | 'ς' | '\u{200C}' | '\u{200D}' => {
                    let mut s = String::with_capacity(domain.len());
                    let tail = prev.as_str();
                    let head = &domain[..domain.len() - tail.len()];
                    s.push_str(head);
                    for c in tail.chars() {
                        match c {
                            'ß' | 'ẞ' => {
                                s.push_str("ss");
                            }
                            'ς' => {
                                s.push('σ');
                            }
                            '\u{200C}' | '\u{200D}' => {}
                            _ => {
                                s.push(c);
                            }
                        }
                    }
                    return Cow::Owned(s);
                }
                _ => {}
            }
        } else {
            break;
        }
    }
    Cow::Borrowed(domain)
}
