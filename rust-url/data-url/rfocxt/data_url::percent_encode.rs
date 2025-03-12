use alloc::{string::String, vec::Vec};
use core::fmt;
fn percent_encode(byte: u8, string: &mut String) {
    const HEX_UPPER: [u8; 16] = *b"0123456789ABCDEF";
    string.push('%');
    string.push(HEX_UPPER[(byte >> 4) as usize] as char);
    string.push(HEX_UPPER[(byte & 0x0f) as usize] as char);
}
