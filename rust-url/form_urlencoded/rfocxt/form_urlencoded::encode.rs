pub type EncodingOverride<'a> = Option<&'a dyn Fn(&str) -> Cow<'_, [u8]>>;
use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::str;
use percent_encoding::{percent_decode, percent_encode_byte};
pub(crate) fn encode<'a>(
    encoding_override: EncodingOverride<'_>,
    input: &'a str,
) -> Cow<'a, [u8]> {
    if let Some(o) = encoding_override {
        return o(input);
    }
    input.as_bytes().into()
}
