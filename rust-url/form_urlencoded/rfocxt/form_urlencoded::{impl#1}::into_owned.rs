pub type EncodingOverride<'a> = Option<&'a dyn Fn(&str) -> Cow<'_, [u8]>>;
use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::str;
use percent_encoding::{percent_decode, percent_encode_byte};
#[derive(Copy, Clone)]
pub struct Parse<'a> {
    input: &'a [u8],
}
pub struct ParseIntoOwned<'a> {
    inner: Parse<'a>,
}
impl<'a> Parse<'a> {
    pub fn into_owned(self) -> ParseIntoOwned<'a> {
        ParseIntoOwned { inner: self }
    }
}
