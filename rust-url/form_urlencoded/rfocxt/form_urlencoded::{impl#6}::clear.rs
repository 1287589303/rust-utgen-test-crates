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
pub struct Serializer<'a, T: Target> {
    target: Option<T>,
    start_position: usize,
    encoding: EncodingOverride<'a>,
}
impl<'a, T: Target> Serializer<'a, T> {
    pub fn new(target: T) -> Self {
        Self::for_suffix(target, 0)
    }
    pub fn for_suffix(mut target: T, start_position: usize) -> Self {
        if target.as_mut_string().len() < start_position {
            panic!(
                "invalid length {} for target of length {}", start_position, target
                .as_mut_string().len()
            );
        }
        Serializer {
            target: Some(target),
            start_position,
            encoding: None,
        }
    }
    pub fn clear(&mut self) -> &mut Self {
        string(&mut self.target).truncate(self.start_position);
        self
    }
    pub fn encoding_override(&mut self, new: EncodingOverride<'a>) -> &mut Self {
        self.encoding = new;
        self
    }
    pub fn append_pair(&mut self, name: &str, value: &str) -> &mut Self {
        append_pair(
            string(&mut self.target),
            self.start_position,
            self.encoding,
            name,
            value,
        );
        self
    }
    pub fn append_key_only(&mut self, name: &str) -> &mut Self {
        append_key_only(
            string(&mut self.target),
            self.start_position,
            self.encoding,
            name,
        );
        self
    }
    pub fn extend_pairs<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        {
            let string = string(&mut self.target);
            for pair in iter {
                let (k, v) = pair.borrow();
                append_pair(
                    string,
                    self.start_position,
                    self.encoding,
                    k.as_ref(),
                    v.as_ref(),
                );
            }
        }
        self
    }
    pub fn extend_keys_only<I, K>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Borrow<K>,
        K: AsRef<str>,
    {
        {
            let string = string(&mut self.target);
            for key in iter {
                let k = key.borrow().as_ref();
                append_key_only(string, self.start_position, self.encoding, k);
            }
        }
        self
    }
    pub fn finish(&mut self) -> T::Finished {}
}
fn string<T: Target>(target: &mut Option<T>) -> &mut String {
    target.as_mut().expect("url::form_urlencoded::Serializer finished").as_mut_string()
}
