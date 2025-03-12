pub type EncodingOverride<'a> = Option<&'a dyn Fn(&str) -> Cow<'_, [u8]>>;
use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::str;
use percent_encoding::{percent_decode, percent_encode_byte};
#[derive(Debug)]
pub struct ByteSerialize<'a> {
    bytes: &'a [u8],
}
impl<'a> Iterator for ByteSerialize<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {}
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.bytes.is_empty() { (0, Some(0)) } else { (1, Some(self.bytes.len())) }
    }
}
