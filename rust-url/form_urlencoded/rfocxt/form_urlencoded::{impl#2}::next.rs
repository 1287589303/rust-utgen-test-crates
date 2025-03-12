pub type EncodingOverride<'a> = Option<&'a dyn Fn(&str) -> Cow<'_, [u8]>>;
use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::str;
use percent_encoding::{percent_decode, percent_encode_byte};
pub struct ParseIntoOwned<'a> {
    inner: Parse<'a>,
}
#[derive(Copy, Clone)]
pub struct Parse<'a> {
    input: &'a [u8],
}
impl Iterator for ParseIntoOwned<'_> {
    type Item = (String, String);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k.into_owned(), v.into_owned()))
    }
}
