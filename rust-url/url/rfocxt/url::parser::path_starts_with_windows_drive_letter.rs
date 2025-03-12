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
fn path_starts_with_windows_drive_letter(s: &str) -> bool {
    if let Some(c) = s.as_bytes().first() {
        matches!(c, b'/' | b'\\' | b'?' | b'#')
            && starts_with_windows_drive_letter(&s[1..])
    } else {
        false
    }
}
fn starts_with_windows_drive_letter(s: &str) -> bool {
    s.len() >= 2 && ascii_alpha(s.as_bytes()[0] as char)
        && matches!(s.as_bytes() [1], b':' | b'|')
        && (s.len() == 2 || matches!(s.as_bytes() [2], b'/' | b'\\' | b'?' | b'#'))
}
