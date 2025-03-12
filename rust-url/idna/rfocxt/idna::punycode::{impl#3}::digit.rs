use alloc::{string::String, vec::Vec};
use core::char;
use core::fmt::Write;
use core::marker::PhantomData;
const BASE: u32 = 36;
const T_MIN: u32 = 1;
const T_MAX: u32 = 26;
const SKEW: u32 = 38;
const DAMP: u32 = 700;
const INITIAL_BIAS: u32 = 72;
const INITIAL_N: u32 = 0x80;
pub(crate) trait PunycodeCodeUnit {
    fn is_delimiter(&self) -> bool;
    fn is_ascii(&self) -> bool;
    fn digit(&self) -> Option<u32>;
    fn char(&self) -> char;
    fn char_ascii_lower_case(&self) -> char;
}
impl PunycodeCodeUnit for char {
    fn is_delimiter(&self) -> bool {}
    fn is_ascii(&self) -> bool {}
    fn digit(&self) -> Option<u32> {
        let byte = *self;
        Some(
            match byte {
                byte @ '0'..='9' => u32::from(byte) - u32::from('0') + 26,
                byte @ 'a'..='z' => u32::from(byte) - u32::from('a'),
                _ => return None,
            },
        )
    }
    fn char(&self) -> char {
        debug_assert!(false);
        *self
    }
    fn char_ascii_lower_case(&self) -> char {}
}
