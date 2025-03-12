use crate::punycode::Decoder;
use crate::punycode::InternalCaller;
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt::Write;
use idna_adapter::*;
use smallvec::SmallVec;
use utf8_iter::Utf8CharsEx;
const PUNYCODE_DECODE_MAX_INPUT_LENGTH: usize = 2000;
const PUNYCODE_ENCODE_MAX_INPUT_LENGTH: usize = 1000;
const UPPER_CASE_MASK: u128 = upper_case_mask();
const GLYPHLESS_MASK: u128 = glyphless_mask();
const DOT_MASK: u128 = 1 << b'.';
const PUNYCODE_PREFIX: u32 = ((b'-' as u32) << 24) | ((b'-' as u32) << 16)
    | ((b'N' as u32) << 8) | b'X' as u32;
const PUNYCODE_PREFIX_MASK: u32 = (0xFF << 24) | (0xFF << 16) | (0xDF << 8) | 0xDF;
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ProcessingError {
    /// There was a validity error according to the chosen options.
    ///
    /// In case of `Operation::ToAscii`, there is no output. Otherwise, output was written to the
    /// sink and the output contains at least one U+FFFD REPLACEMENT CHARACTER to denote an error.
    ValidityError,
    /// The sink emitted [`core::fmt::Error`]. The partial output written to the sink must not
    /// be used.
    SinkError,
}
pub(crate) enum PunycodeEncodeError {
    Overflow,
    Sink,
}
impl From<crate::punycode::PunycodeEncodeError> for ProcessingError {
    fn from(_: crate::punycode::PunycodeEncodeError) -> Self {
        unreachable!(
            "Punycode overflows should not be possible due to PUNYCODE_ENCODE_MAX_INPUT_LENGTH"
        );
    }
}
