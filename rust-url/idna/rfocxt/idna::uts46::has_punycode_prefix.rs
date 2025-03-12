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
#[inline(always)]
fn has_punycode_prefix(slice: &[u8]) -> bool {
    if slice.len() < 4 {
        return false;
    }
    let a = slice[0];
    let b = slice[1];
    let c = slice[2];
    let d = slice[3];
    let u = (u32::from(d) << 24) | (u32::from(c) << 16) | (u32::from(b) << 8)
        | u32::from(a);
    (u & PUNYCODE_PREFIX_MASK) == PUNYCODE_PREFIX
}
