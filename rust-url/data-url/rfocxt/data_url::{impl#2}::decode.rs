use alloc::{string::String, vec::Vec};
use core::fmt;
pub struct DataUrl<'a> {
    mime_type: mime::Mime,
    base64: bool,
    encoded_body_plus_fragment: &'a str,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Mime {
    pub type_: String,
    pub subtype: String,
    /// (name, value)
    pub parameters: Vec<(String, String)>,
}
pub struct FragmentIdentifier<'a>(&'a str);
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
    {
        if self.base64 {
            decode_with_base64(self.encoded_body_plus_fragment, write_body_bytes)
        } else {
            decode_without_base64(self.encoded_body_plus_fragment, write_body_bytes)
                .map_err(forgiving_base64::DecodeError::WriteError)
        }
    }
    pub fn decode_to_vec(
        &self,
    ) -> Result<
        (Vec<u8>, Option<FragmentIdentifier<'a>>),
        forgiving_base64::InvalidBase64,
    > {}
}
fn decode_without_base64<F, E>(
    encoded_body_plus_fragment: &str,
    mut write_bytes: F,
) -> Result<Option<FragmentIdentifier<'_>>, E>
where
    F: FnMut(&[u8]) -> Result<(), E>,
{
    let bytes = encoded_body_plus_fragment.as_bytes();
    let mut slice_start = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        if matches!(byte, b'%' | b'#' | b'\t' | b'\n' | b'\r') {
            if i > slice_start {
                write_bytes(&bytes[slice_start..i])?;
                slice_start = i;
            }
            match byte {
                b'%' => {
                    let l = bytes.get(i + 2).and_then(|&b| (b as char).to_digit(16));
                    let h = bytes.get(i + 1).and_then(|&b| (b as char).to_digit(16));
                    if let (Some(h), Some(l)) = (h, l) {
                        let one_byte = h as u8 * 0x10 + l as u8;
                        write_bytes(&[one_byte])?;
                        slice_start = i + 3;
                    } else {}
                }
                b'#' => {
                    let fragment_start = i + 1;
                    let fragment = &encoded_body_plus_fragment[fragment_start..];
                    return Ok(Some(FragmentIdentifier(fragment)));
                }
                _ => slice_start = i + 1,
            }
        }
    }
    write_bytes(&bytes[slice_start..])?;
    Ok(None)
}
fn decode_with_base64<F, E>(
    encoded_body_plus_fragment: &str,
    write_bytes: F,
) -> Result<Option<FragmentIdentifier<'_>>, forgiving_base64::DecodeError<E>>
where
    F: FnMut(&[u8]) -> Result<(), E>,
{
    let mut decoder = forgiving_base64::Decoder::new(write_bytes);
    let fragment = decode_without_base64(
        encoded_body_plus_fragment,
        |bytes| decoder.feed(bytes),
    )?;
    decoder.finish()?;
    Ok(fragment)
}
