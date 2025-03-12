pub use crate::{
    error::Error, parser::{parse, Parser, ParserBuilder},
    unicode::UnicodeWordError,
};
use alloc::string::String;
pub fn is_word_byte(c: u8) -> bool {
    match c {
        b'_' | b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => true,
        _ => false,
    }
}
