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
fn apply_ascii_deny_list_to_potentially_upper_case_ascii(
    b: u8,
    deny_list: u128,
) -> char {
    if (deny_list & (1u128 << b)) == 0 {
        return char::from(b);
    }
    if in_inclusive_range8(b, b'A', b'Z') {
        return char::from(b + 0x20);
    }
    '\u{FFFD}'
}
#[inline(always)]
fn in_inclusive_range8(u: u8, start: u8, end: u8) -> bool {
    u.wrapping_sub(start) <= (end - start)
}
