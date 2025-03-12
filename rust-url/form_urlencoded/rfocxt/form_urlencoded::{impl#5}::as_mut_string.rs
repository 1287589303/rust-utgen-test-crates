pub type EncodingOverride<'a> = Option<&'a dyn Fn(&str) -> Cow<'_, [u8]>>;
use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::str;
use percent_encoding::{percent_decode, percent_encode_byte};
pub trait Target {
    type Finished;
    fn as_mut_string(&mut self) -> &mut String;
    fn finish(self) -> Self::Finished;
}
impl Target for &mut String {
    type Finished = Self;
    fn as_mut_string(&mut self) -> &mut String {
        self
    }
    fn finish(self) -> Self {
        self
    }
}
