use alloc::{string::String, vec::Vec};
use core::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct Mime {
    pub type_: String,
    pub subtype: String,
    /// (name, value)
    pub parameters: Vec<(String, String)>,
}
#[derive(Debug)]
pub struct MimeParsingError(());
fn parse_header(from_colon_to_comma: &str) -> (mime::Mime, bool) {
    let trimmed = from_colon_to_comma
        .trim_matches(|c| matches!(c, ' ' | '\t' | '\n' | '\r'));
    let without_base64_suffix = remove_base64_suffix(trimmed);
    let base64 = without_base64_suffix.is_some();
    let mime_type = without_base64_suffix.unwrap_or(trimmed);
    let mut string = String::new();
    if mime_type.starts_with(';') {
        string.push_str("text/plain")
    }
    let mut in_query = false;
    for byte in mime_type.bytes() {
        match byte {
            b'\t' | b'\n' | b'\r' => continue,
            b'\0'..=b'\x1F' | b'\x7F'..=b'\xFF' => percent_encode(byte, &mut string),
            b' ' | b'"' | b'<' | b'>' if in_query => percent_encode(byte, &mut string),
            b'?' => {
                in_query = true;
                string.push('?')
            }
            _ => string.push(byte as char),
        }
    }
    let mime_type = string
        .parse()
        .unwrap_or_else(|_| mime::Mime {
            type_: String::from("text"),
            subtype: String::from("plain"),
            parameters: vec![(String::from("charset"), String::from("US-ASCII"))],
        });
    (mime_type, base64)
}
#[allow(clippy::skip_while_next)]
fn remove_base64_suffix(s: &str) -> Option<&str> {
    let mut bytes = s.bytes();
    {
        let iter = bytes.by_ref().filter(|&byte| !matches!(byte, b'\t' | b'\n' | b'\r'));
        let mut iter = iter.rev();
        require!(iter.next() ? == b'4');
        require!(iter.next() ? == b'6');
        require!(iter.next() ?.eq_ignore_ascii_case(& b'e'));
        require!(iter.next() ?.eq_ignore_ascii_case(& b's'));
        require!(iter.next() ?.eq_ignore_ascii_case(& b'a'));
        require!(iter.next() ?.eq_ignore_ascii_case(& b'b'));
        require!(iter.skip_while(|& byte | byte == b' ').next() ? == b';');
    }
    Some(&s[..bytes.len()])
}
fn percent_encode(byte: u8, string: &mut String) {
    const HEX_UPPER: [u8; 16] = *b"0123456789ABCDEF";
    string.push('%');
    string.push(HEX_UPPER[(byte >> 4) as usize] as char);
    string.push(HEX_UPPER[(byte & 0x0f) as usize] as char);
}
