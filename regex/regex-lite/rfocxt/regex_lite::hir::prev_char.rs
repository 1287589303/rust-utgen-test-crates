use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
fn prev_char(ch: char) -> Option<char> {
    if ch == '\u{E000}' {
        return Some('\u{D7FF}');
    }
    Some(char::from_u32(u32::from(ch).checked_sub(1)?).unwrap())
}
