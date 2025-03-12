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
#[inline]
fn value_to_digit(value: u32) -> char {
    match value {
        0..=25 => (value as u8 + b'a') as char,
        26..=35 => (value as u8 - 26 + b'0') as char,
        _ => panic!(),
    }
}
