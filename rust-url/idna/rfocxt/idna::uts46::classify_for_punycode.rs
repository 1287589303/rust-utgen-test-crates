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
#[derive(PartialEq, Eq, Copy, Clone)]
enum PunycodeClassification {
    Ascii,
    Unicode,
    Error,
}
#[inline(always)]
fn classify_for_punycode(label: &[char]) -> PunycodeClassification {
    let mut iter = label.iter().copied();
    loop {
        if let Some(c) = iter.next() {
            if c.is_ascii() {
                continue;
            }
            if c == '\u{FFFD}' {
                return PunycodeClassification::Error;
            }
            for c in iter {
                if c == '\u{FFFD}' {
                    return PunycodeClassification::Error;
                }
            }
            return PunycodeClassification::Unicode;
        }
        return PunycodeClassification::Ascii;
    }
}
