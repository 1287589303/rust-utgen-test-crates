pub type ParseResult<T> = Result<T, ParseError>;
use alloc::string::String;
use alloc::string::ToString;
use core::fmt::{self, Formatter, Write};
use core::str;
use crate::host::{Host, HostInternal};
use crate::Url;
use form_urlencoded::EncodingOverride;
use percent_encoding::{percent_encode, utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const PATH: &AsciiSet = &FRAGMENT.add(b'#').add(b'?').add(b'{').add(b'}');
pub(crate) const USERINFO: &AsciiSet = &PATH
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'|');
pub(crate) const PATH_SEGMENT: &AsciiSet = &PATH.add(b'/').add(b'%');
pub(crate) const SPECIAL_PATH_SEGMENT: &AsciiSet = &PATH_SEGMENT.add(b'\\');
const QUERY: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
const SPECIAL_QUERY: &AsciiSet = &QUERY.add(b'\'');
#[derive(Clone, Debug)]
pub struct Input<'i> {
    chars: str::Chars<'i>,
}
impl<'i> Input<'i> {
    pub fn new_no_trim(input: &'i str) -> Self {
        Input { chars: input.chars() }
    }
    pub fn new_trim_tab_and_newlines(
        original_input: &'i str,
        vfn: Option<&dyn Fn(SyntaxViolation)>,
    ) -> Self {
        let input = original_input.trim_matches(ascii_tab_or_new_line);
        if let Some(vfn) = vfn {
            if input.len() < original_input.len() {
                vfn(SyntaxViolation::C0SpaceIgnored)
            }
            if input.chars().any(ascii_tab_or_new_line) {
                vfn(SyntaxViolation::TabOrNewlineIgnored)
            }
        }
        Input { chars: input.chars() }
    }
    pub fn new_trim_c0_control_and_space(
        original_input: &'i str,
        vfn: Option<&dyn Fn(SyntaxViolation)>,
    ) -> Self {
        let input = original_input.trim_matches(c0_control_or_space);
        if let Some(vfn) = vfn {
            if input.len() < original_input.len() {
                vfn(SyntaxViolation::C0SpaceIgnored)
            }
            if input.chars().any(ascii_tab_or_new_line) {
                vfn(SyntaxViolation::TabOrNewlineIgnored)
            }
        }
        Input { chars: input.chars() }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    fn starts_with<P: Pattern>(&self, p: P) -> bool {}
    #[inline]
    pub fn split_prefix<P: Pattern>(&self, p: P) -> Option<Self> {
        let mut remaining = self.clone();
        if p.split_prefix(&mut remaining) { Some(remaining) } else { None }
    }
    #[inline]
    fn split_first(&self) -> (Option<char>, Self) {
        let mut remaining = self.clone();
        (remaining.next(), remaining)
    }
    #[inline]
    fn count_matching<F: Fn(char) -> bool>(&self, f: F) -> (u32, Self) {
        let mut count = 0;
        let mut remaining = self.clone();
        loop {
            let mut input = remaining.clone();
            if matches!(input.next(), Some(c) if f(c)) {
                remaining = input;
                count += 1;
            } else {
                return (count, remaining);
            }
        }
    }
    #[inline]
    fn next_utf8(&mut self) -> Option<(char, &'i str)> {}
}
