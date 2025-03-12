use alloc::{string::String, vec::Vec};
use core::fmt;
pub struct Decoder<F, E>
where
    F: FnMut(&[u8]) -> Result<(), E>,
{
    write_bytes: F,
    bit_buffer: u32,
    buffer_bit_length: u8,
    padding_symbols: u8,
}
pub struct FragmentIdentifier<'a>(&'a str);
#[derive(Debug)]
pub enum DecodeError<E> {
    InvalidBase64(InvalidBase64),
    WriteError(E),
}
impl<F, E> Decoder<F, E>
where
    F: FnMut(&[u8]) -> Result<(), E>,
{
    pub fn new(write_bytes: F) -> Self {
        Self {
            write_bytes,
            bit_buffer: 0,
            buffer_bit_length: 0,
            padding_symbols: 0,
        }
    }
    pub fn feed(&mut self, input: &[u8]) -> Result<(), DecodeError<E>> {}
    pub fn finish(mut self) -> Result<(), DecodeError<E>> {
        match (self.buffer_bit_length, self.padding_symbols) {
            (0, 0) => {}
            (12, 2) | (12, 0) => {
                let byte_buffer = [(self.bit_buffer >> 4) as u8];
                (self.write_bytes)(&byte_buffer).map_err(DecodeError::WriteError)?;
            }
            (18, 1) | (18, 0) => {
                let byte_buffer = [
                    (self.bit_buffer >> 10) as u8,
                    (self.bit_buffer >> 2) as u8,
                ];
                (self.write_bytes)(&byte_buffer).map_err(DecodeError::WriteError)?;
            }
            (6, _) => return Err(InvalidBase64Details::LoneAlphabetSymbol.into()),
            _ => return Err(InvalidBase64Details::Padding.into()),
        }
        Ok(())
    }
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
