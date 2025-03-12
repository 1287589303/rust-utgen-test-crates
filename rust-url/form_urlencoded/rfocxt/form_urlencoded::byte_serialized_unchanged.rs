pub type EncodingOverride<'a> = Option<&'a dyn Fn(&str) -> Cow<'_, [u8]>>;
use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::str;
use percent_encoding::{percent_decode, percent_encode_byte};
fn byte_serialized_unchanged(byte: u8) -> bool {
    matches!(
        byte, b'*' | b'-' | b'.' | b'0'..= b'9' | b'A'..= b'Z' | b'_' | b'a'..= b'z'
    )
}
