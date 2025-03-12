use core::{cmp, mem, num::NonZeroUsize};
use alloc::{vec, vec::Vec};
use crate::hir::{self, Hir};
pub fn rank(byte: u8) -> u8 {
    crate::rank::BYTE_FREQUENCIES[usize::from(byte)]
}
