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
fn check_url_code_point(vfn: &dyn Fn(SyntaxViolation), c: char, input: &Input<'_>) {
    if c == '%' {
        let mut input = input.clone();
        if !matches!(
            (input.next(), input.next()), (Some(a), Some(b)) if a.is_ascii_hexdigit() &&
            b.is_ascii_hexdigit()
        ) {
            vfn(SyntaxViolation::PercentDecode)
        }
    } else if !is_url_code_point(c) {
        vfn(SyntaxViolation::NonUrlCodePoint)
    }
}
#[inline]
fn is_url_code_point(c: char) -> bool {
    matches!(
        c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '!' | '$' | '&' | '\'' | '(' | ')' | '*' |
        '+' | ',' | '-' | '.' | '/' | ':' | ';' | '=' | '?' | '@' | '_' | '~' | '\u{A0}'
        ..='\u{D7FF}' | '\u{E000}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}' | '\u{10000}'
        ..='\u{1FFFD}' | '\u{20000}'..='\u{2FFFD}' | '\u{30000}'..='\u{3FFFD}' |
        '\u{40000}'..='\u{4FFFD}' | '\u{50000}'..='\u{5FFFD}' | '\u{60000}'..='\u{6FFFD}'
        | '\u{70000}'..='\u{7FFFD}' | '\u{80000}'..='\u{8FFFD}' | '\u{90000}'
        ..='\u{9FFFD}' | '\u{A0000}'..='\u{AFFFD}' | '\u{B0000}'..='\u{BFFFD}' |
        '\u{C0000}'..='\u{CFFFD}' | '\u{D0000}'..='\u{DFFFD}' | '\u{E1000}'..='\u{EFFFD}'
        | '\u{F0000}'..='\u{FFFFD}' | '\u{100000}'..='\u{10FFFD}'
    )
}
