#[cfg(any(feature = "alloc", test))]
use crate::chunked_encoder;
use crate::{
    encode::{encode_with_padding, EncodeSliceError},
    encoded_len, DecodeError, DecodeSliceError,
};
#[cfg(any(feature = "alloc", test))]
use alloc::vec::Vec;
#[cfg(any(feature = "alloc", test))]
use alloc::{string::String, vec};
pub use general_purpose::{GeneralPurpose, GeneralPurposeConfig};
pub trait Engine: Send + Sync {
    type Config: Config;
    type DecodeEstimate: DecodeEstimate;
    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize;
    fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate;
    fn internal_decode(
        &self,
        input: &[u8],
        output: &mut [u8],
        decode_estimate: Self::DecodeEstimate,
    ) -> Result<DecodeMetadata, DecodeSliceError>;
    fn config(&self) -> &Self::Config;
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn encode<T: AsRef<[u8]>>(&self, input: T) -> String;
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn encode_string<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut String) {
        fn inner<E>(engine: &E, input_bytes: &[u8], output_buf: &mut String)
        where
            E: Engine + ?Sized,
        {
            let mut sink = chunked_encoder::StringSink::new(output_buf);
            chunked_encoder::ChunkedEncoder::new(engine)
                .encode(input_bytes, &mut sink)
                .expect("Writing to a String shouldn't fail");
        }
        inner(self, input.as_ref(), output_buf);
    }
    #[cfg_attr(feature = "alloc", doc = "```")]
    #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
    #[inline]
    fn encode_slice<T: AsRef<[u8]>>(
        &self,
        input: T,
        output_buf: &mut [u8],
    ) -> Result<usize, EncodeSliceError>;
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn decode<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, DecodeError>;
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn decode_vec<T: AsRef<[u8]>>(
        &self,
        input: T,
        buffer: &mut Vec<u8>,
    ) -> Result<(), DecodeError>;
    #[inline]
    fn decode_slice<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeSliceError>;
    #[inline]
    fn decode_slice_unchecked<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeError>;
}
fn inner<E>(engine: &E, input_bytes: &[u8], output_buf: &mut String)
where
    E: Engine + ?Sized,
{
    let mut sink = chunked_encoder::StringSink::new(output_buf);
    chunked_encoder::ChunkedEncoder::new(engine)
        .encode(input_bytes, &mut sink)
        .expect("Writing to a String shouldn't fail");
}
