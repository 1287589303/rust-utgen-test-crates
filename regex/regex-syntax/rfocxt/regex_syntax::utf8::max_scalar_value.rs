use core::{char, fmt, iter::FusedIterator, slice};
use alloc::{vec, vec::Vec};
const MAX_UTF8_BYTES: usize = 4;
fn max_scalar_value(nbytes: usize) -> u32 {
    match nbytes {
        1 => 0x007F,
        2 => 0x07FF,
        3 => 0xFFFF,
        4 => 0x0010_FFFF,
        _ => unreachable!("invalid UTF-8 byte sequence size"),
    }
}
