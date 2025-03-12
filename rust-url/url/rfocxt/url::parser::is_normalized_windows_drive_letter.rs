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
fn is_normalized_windows_drive_letter(segment: &str) -> bool {
    is_windows_drive_letter(segment) && segment.as_bytes()[1] == b':'
}
#[inline]
pub fn is_windows_drive_letter(segment: &str) -> bool {
    segment.len() == 2 && starts_with_windows_drive_letter(segment)
}
