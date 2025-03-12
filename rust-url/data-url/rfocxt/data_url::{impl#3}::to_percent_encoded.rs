use alloc::{string::String, vec::Vec};
use core::fmt;
pub struct FragmentIdentifier<'a>(&'a str);
impl FragmentIdentifier<'_> {
    pub fn to_percent_encoded(&self) -> String {
        let mut string = String::new();
        for byte in self.0.bytes() {
            match byte {
                b'\t' | b'\n' | b'\r' => continue,
                b'\0'..=b' ' | b'"' | b'<' | b'>' | b'`' | b'\x7F'..=b'\xFF' => {
                    percent_encode(byte, &mut string)
                }
                _ => string.push(byte as char),
            }
        }
        string
    }
}
fn percent_encode(byte: u8, string: &mut String) {
    const HEX_UPPER: [u8; 16] = *b"0123456789ABCDEF";
    string.push('%');
    string.push(HEX_UPPER[(byte >> 4) as usize] as char);
    string.push(HEX_UPPER[(byte & 0x0f) as usize] as char);
}
