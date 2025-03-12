pub type EncodingOverride<'a> = Option<&'a dyn Fn(&str) -> Cow<'_, [u8]>>;
use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::str;
use percent_encoding::{percent_decode, percent_encode_byte};
#[derive(Debug)]
pub struct ByteSerialize<'a> {
    bytes: &'a [u8],
}
pub fn byte_serialize(input: &[u8]) -> ByteSerialize<'_> {
    ByteSerialize { bytes: input }
}
