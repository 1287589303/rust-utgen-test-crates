pub type EncodingOverride<'a> = Option<&'a dyn Fn(&str) -> Cow<'_, [u8]>>;
use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::str;
use percent_encoding::{percent_decode, percent_encode_byte};
#[derive(Copy, Clone)]
pub struct Parse<'a> {
    input: &'a [u8],
}
#[inline]
pub fn parse(input: &[u8]) -> Parse<'_> {
    Parse { input }
}
