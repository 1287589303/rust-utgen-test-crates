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
pub fn verify_dns_length(domain_name: &str, allow_trailing_dot: bool) -> bool {
    let bytes = domain_name.as_bytes();
    debug_assert!(bytes.is_ascii());
    let domain_name_without_trailing_dot = if let Some(without) = bytes
        .strip_suffix(b".")
    {
        if !allow_trailing_dot {
            return false;
        }
        without
    } else {
        bytes
    };
    if domain_name_without_trailing_dot.len() > 253 {
        return false;
    }
    for label in domain_name_without_trailing_dot.split(|b| *b == b'.') {
        if label.is_empty() {
            return false;
        }
        if label.len() > 63 {
            return false;
        }
    }
    true
}
