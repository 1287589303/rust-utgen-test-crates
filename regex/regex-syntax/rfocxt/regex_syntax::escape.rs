pub use crate::{
    error::Error, parser::{parse, Parser, ParserBuilder},
    unicode::UnicodeWordError,
};
use alloc::string::String;
pub fn escape(text: &str) -> String {
    let mut quoted = String::new();
    escape_into(text, &mut quoted);
    quoted
}
pub fn escape_into(text: &str, buf: &mut String) {
    buf.reserve(text.len());
    for c in text.chars() {
        if is_meta_character(c) {
            buf.push('\\');
        }
        buf.push(c);
    }
}
