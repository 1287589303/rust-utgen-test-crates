use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
fn is_escapeable_character(c: char) -> bool {
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
fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^'
        | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}
