pub type EncodingOverride<'a> = Option<&'a dyn Fn(&str) -> Cow<'_, [u8]>>;
use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::str;
use percent_encoding::{percent_decode, percent_encode_byte};
fn append_key_only(
    string: &mut String,
    start_position: usize,
    encoding: EncodingOverride,
    name: &str,
) {
    append_separator_if_needed(string, start_position);
    append_encoded(name, string, encoding);
}
fn append_encoded(s: &str, string: &mut String, encoding: EncodingOverride<'_>) {
    string.extend(byte_serialize(&encode(encoding, s)))
}
fn append_separator_if_needed(string: &mut String, start_position: usize) {
    if string.len() > start_position {
        string.push('&')
    }
}
