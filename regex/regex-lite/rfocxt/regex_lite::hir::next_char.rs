use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
fn next_char(ch: char) -> Option<char> {
    if ch == '\u{D7FF}' {
        return Some('\u{E000}');
    }
    char::from_u32(u32::from(ch).checked_add(1).unwrap())
}
