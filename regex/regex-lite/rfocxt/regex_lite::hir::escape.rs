use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
pub fn escape(pattern: &str) -> String {
    let mut buf = String::new();
    buf.reserve(pattern.len());
    for ch in pattern.chars() {
        if is_meta_character(ch) {
            buf.push('\\');
        }
        buf.push(ch);
    }
    buf
}
fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^'
        | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}
