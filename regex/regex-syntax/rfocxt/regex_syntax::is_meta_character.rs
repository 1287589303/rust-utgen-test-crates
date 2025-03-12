pub use crate::{
    error::Error, parser::{parse, Parser, ParserBuilder},
    unicode::UnicodeWordError,
};
use alloc::string::String;
pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^'
        | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}
