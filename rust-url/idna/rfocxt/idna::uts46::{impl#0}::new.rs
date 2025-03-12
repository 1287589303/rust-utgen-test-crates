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
#[repr(transparent)]
pub struct AsciiDenyList {
    bits: u128,
}
impl AsciiDenyList {
    pub const EMPTY: AsciiDenyList = AsciiDenyList::new(false, "");
    pub const STD3: AsciiDenyList = AsciiDenyList { bits: ldh_mask() };
    pub const URL: AsciiDenyList = AsciiDenyList::new(true, "%#/:<>?@[\\]^|");
    pub const fn new(deny_glyphless: bool, deny_list: &str) -> Self {
        let mut bits = UPPER_CASE_MASK;
        if deny_glyphless {
            bits |= GLYPHLESS_MASK;
        }
        let mut i = 0;
        let bytes = deny_list.as_bytes();
        while i < bytes.len() {
            let b = bytes[i];
            assert!(b < 0x80, "ASCII deny list must be ASCII.");
            assert!(b != b'.', "ASCII deny list must not contain the dot.");
            assert!(b != b'-', "ASCII deny list must not contain the hyphen.");
            assert!(
                ! ((b >= b'0') && (b <= b'9')),
                "ASCII deny list must not contain digits."
            );
            assert!(
                ! ((b >= b'a') && (b <= b'z')),
                "ASCII deny list must not contain letters."
            );
            assert!(
                ! ((b >= b'A') && (b <= b'Z')),
                "ASCII deny list must not contain letters."
            );
            bits |= 1u128 << b;
            i += 1;
        }
        AsciiDenyList { bits }
    }
}
