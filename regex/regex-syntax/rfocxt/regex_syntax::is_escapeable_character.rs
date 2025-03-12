pub use crate::{
    error::Error, parser::{parse, Parser, ParserBuilder},
    unicode::UnicodeWordError,
};
use alloc::string::String;
pub fn is_escapeable_character(c: char) -> bool {
    if is_meta_character(c) {
        return true;
    }
    if !c.is_ascii() {
        return false;
    }
    match c {
        '0'..='9' | 'A'..='Z' | 'a'..='z' => false,
        '<' | '>' => false,
        _ => true,
    }
}
pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^'
        | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}
