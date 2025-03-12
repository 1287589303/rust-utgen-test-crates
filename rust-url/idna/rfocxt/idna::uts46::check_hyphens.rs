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
fn check_hyphens(
    mut_label: &mut [char],
    allow_third_fourth: bool,
    fail_fast: bool,
    had_errors: &mut bool,
) -> bool {
    if let Some(first) = mut_label.first_mut() {
        if *first == '-' {
            if fail_fast {
                return true;
            }
            *had_errors = true;
            *first = '\u{FFFD}';
        }
    }
    if let Some(last) = mut_label.last_mut() {
        if *last == '-' {
            if fail_fast {
                return true;
            }
            *had_errors = true;
            *last = '\u{FFFD}';
        }
    }
    if allow_third_fourth {
        return false;
    }
    if mut_label.len() >= 4 && mut_label[2] == '-' && mut_label[3] == '-' {
        if fail_fast {
            return true;
        }
        *had_errors = true;
        mut_label[2] = '\u{FFFD}';
        mut_label[3] = '\u{FFFD}';
    }
    false
}
