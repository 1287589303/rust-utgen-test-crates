use alloc::{string::String, vec::Vec};
use core::fmt;
pub struct DataUrl<'a> {
    mime_type: mime::Mime,
    base64: bool,
    encoded_body_plus_fragment: &'a str,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Mime {
    pub type_: String,
    pub subtype: String,
    /// (name, value)
    pub parameters: Vec<(String, String)>,
}
#[derive(Debug)]
pub enum DataUrlError {
    NotADataUrl,
    NoComma,
}
impl<'a> DataUrl<'a> {
    pub fn process(input: &'a str) -> Result<Self, DataUrlError> {
        use crate::DataUrlError::*;
        let after_colon = pretend_parse_data_url(input).ok_or(NotADataUrl)?;
        let (from_colon_to_comma, encoded_body_plus_fragment) = find_comma_before_fragment(
                after_colon,
            )
            .ok_or(NoComma)?;
        let (mime_type, base64) = parse_header(from_colon_to_comma);
        Ok(DataUrl {
            mime_type,
            base64,
            encoded_body_plus_fragment,
        })
    }
    pub fn mime_type(&self) -> &mime::Mime {}
    pub fn decode<F, E>(
        &self,
        write_body_bytes: F,
    ) -> Result<Option<FragmentIdentifier<'a>>, forgiving_base64::DecodeError<E>>
    where
        F: FnMut(&[u8]) -> Result<(), E>,
    {}
    pub fn decode_to_vec(
        &self,
    ) -> Result<
        (Vec<u8>, Option<FragmentIdentifier<'a>>),
        forgiving_base64::InvalidBase64,
    > {}
}
fn find_comma_before_fragment(after_colon: &str) -> Option<(&str, &str)> {
    for (i, byte) in after_colon.bytes().enumerate() {
        if byte == b',' {
            return Some((&after_colon[..i], &after_colon[i + 1..]));
        }
        if byte == b'#' {
            break;
        }
    }
    None
}
fn pretend_parse_data_url(input: &str) -> Option<&str> {
    let left_trimmed = input.trim_start_matches(|ch| ch <= ' ');
    let mut bytes = left_trimmed.bytes();
    {
        let mut iter = bytes
            .by_ref()
            .filter(|&byte| !matches!(byte, b'\t' | b'\n' | b'\r'));
        require!(iter.next() ?.eq_ignore_ascii_case(& b'd'));
        require!(iter.next() ?.eq_ignore_ascii_case(& b'a'));
        require!(iter.next() ?.eq_ignore_ascii_case(& b't'));
        require!(iter.next() ?.eq_ignore_ascii_case(& b'a'));
        require!(iter.next() ? == b':');
    }
    let bytes_consumed = left_trimmed.len() - bytes.len();
    let after_colon = &left_trimmed[bytes_consumed..];
    Some(after_colon.trim_end_matches(|ch| ch <= ' '))
}
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
