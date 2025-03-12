pub type EncodingOverride<'a> = Option<&'a dyn Fn(&str) -> Cow<'_, [u8]>>;
use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::str;
use percent_encoding::{percent_decode, percent_encode_byte};
fn string<T: Target>(target: &mut Option<T>) -> &mut String {
    target.as_mut().expect("url::form_urlencoded::Serializer finished").as_mut_string()
}
