use alloc::{string::String, vec::Vec};
use core::fmt;
pub struct DataUrl<'a> {
    mime_type: mime::Mime,
    base64: bool,
    encoded_body_plus_fragment: &'a str,
}
#[derive(Debug)]
pub struct InvalidBase64(InvalidBase64Details);
#[derive(Debug, PartialEq, Eq)]
pub struct Mime {
    pub type_: String,
    pub subtype: String,
    /// (name, value)
    pub parameters: Vec<(String, String)>,
}
pub struct FragmentIdentifier<'a>(&'a str);
pub(crate) enum Impossible {}
#[derive(Debug)]
pub enum DecodeError<E> {
    InvalidBase64(InvalidBase64),
    WriteError(E),
}
impl<'a> DataUrl<'a> {
    pub fn process(input: &'a str) -> Result<Self, DataUrlError> {
        use crate::DataUrlError::*;
        let after_colon = pretend_parse_data_url(input).ok_or(NotADataUrl)?;
        let (from_colon_to_comma, encoded_body_plus_fragment) = find_comma_before_fragment(
                after_colon,
            )
            .ok_or(NoComma)?;
        let (mime_type, base64) = parse_header(from_colon_to_comma);
        Ok(DataUrl {
            mime_type,
            base64,
            encoded_body_plus_fragment,
        })
    }
    pub fn mime_type(&self) -> &mime::Mime {}
    pub fn decode<F, E>(
        &self,
        write_body_bytes: F,
    ) -> Result<Option<FragmentIdentifier<'a>>, forgiving_base64::DecodeError<E>>
    where
        F: FnMut(&[u8]) -> Result<(), E>,
    {}
    pub fn decode_to_vec(
        &self,
    ) -> Result<
        (Vec<u8>, Option<FragmentIdentifier<'a>>),
        forgiving_base64::InvalidBase64,
    > {
        let mut body = Vec::new();
        let fragment = self
            .decode(|bytes| {
                body.extend_from_slice(bytes);
                Ok(())
            })?;
        Ok((body, fragment))
    }
}
