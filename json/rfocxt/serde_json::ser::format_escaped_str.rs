use crate::error::{Error, ErrorCode, Result};
use crate::io;
use alloc::string::String;
#[cfg(feature = "raw_value")]
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::{self, Display};
use core::num::FpCategory;
use serde::ser::{self, Impossible, Serialize};
static ESCAPE: [u8; 256] = [
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    BB,
    TT,
    NN,
    UU,
    FF,
    RR,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    __,
    __,
    QU,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    BS,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
];
const BB: u8 = b'b';
const TT: u8 = b't';
const NN: u8 = b'n';
const FF: u8 = b'f';
const RR: u8 = b'r';
const QU: u8 = b'"';
const BS: u8 = b'\\';
const UU: u8 = b'u';
const __: u8 = 0;
fn format_escaped_str<W, F>(
    writer: &mut W,
    formatter: &mut F,
    value: &str,
) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter,
{
    tri!(formatter.begin_string(writer));
    tri!(format_escaped_str_contents(writer, formatter, value));
    formatter.end_string(writer)
}
fn format_escaped_str_contents<W, F>(
    writer: &mut W,
    formatter: &mut F,
    value: &str,
) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter,
{
    let bytes = value.as_bytes();
    let mut start = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        let escape = ESCAPE[byte as usize];
        if escape == 0 {
            continue;
        }
        if start < i {
            tri!(formatter.write_string_fragment(writer, & value[start..i]));
        }
        let char_escape = CharEscape::from_escape_table(escape, byte);
        tri!(formatter.write_char_escape(writer, char_escape));
        start = i + 1;
    }
    if start == bytes.len() {
        return Ok(());
    }
    formatter.write_string_fragment(writer, &value[start..])
}
