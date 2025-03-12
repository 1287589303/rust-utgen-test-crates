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
    fn next(&mut self) -> Option<&'a str> {
        if let Some((&first, tail)) = self.bytes.split_first() {
            if !byte_serialized_unchanged(first) {
                self.bytes = tail;
                return Some(
                    if first == b' ' { "+" } else { percent_encode_byte(first) },
                );
            }
            let position = tail.iter().position(|&b| !byte_serialized_unchanged(b));
            let (unchanged_slice, remaining) = match position {
                Some(i) => self.bytes.split_at(1 + i),
                None => (self.bytes, &[][..]),
            };
            self.bytes = remaining;
            Some(unsafe { str::from_utf8_unchecked(unchanged_slice) })
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {}
}
fn byte_serialized_unchanged(byte: u8) -> bool {
    matches!(
        byte, b'*' | b'-' | b'.' | b'0'..= b'9' | b'A'..= b'Z' | b'_' | b'a'..= b'z'
    )
}
