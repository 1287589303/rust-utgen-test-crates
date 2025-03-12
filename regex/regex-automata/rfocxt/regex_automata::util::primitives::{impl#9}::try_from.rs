use core::num::NonZeroUsize;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use crate::util::int::{Usize, U16, U32, U64};
pub(crate) trait U32 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn low_u16(self) -> u16;
    fn high_u16(self) -> u16;
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SmallIndex(u32);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmallIndexError {
    attempted: u64,
}
impl TryFrom<usize> for SmallIndex {
    type Error = SmallIndexError;
    fn try_from(index: usize) -> Result<SmallIndex, SmallIndexError> {
        if index > SmallIndex::MAX.as_usize() {
            return Err(SmallIndexError {
                attempted: index.as_u64(),
            });
        }
        Ok(SmallIndex::new_unchecked(index))
    }
}
impl SmallIndex {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    pub const MAX: SmallIndex = SmallIndex::new_unchecked(core::i32::MAX as usize - 1);
    #[cfg(target_pointer_width = "16")]
    pub const MAX: SmallIndex = SmallIndex::new_unchecked(core::isize::MAX - 1);
    pub const LIMIT: usize = SmallIndex::MAX.as_usize() + 1;
    pub const ZERO: SmallIndex = SmallIndex::new_unchecked(0);
    pub const SIZE: usize = core::mem::size_of::<SmallIndex>();
    #[inline]
    pub fn new(index: usize) -> Result<SmallIndex, SmallIndexError> {}
    #[inline]
    pub const fn new_unchecked(index: usize) -> SmallIndex {
        SmallIndex(index as u32)
    }
    #[inline]
    pub fn must(index: usize) -> SmallIndex {}
    #[inline]
    pub const fn as_usize(&self) -> usize {}
    #[inline]
    pub const fn as_u64(&self) -> u64 {}
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
    #[inline]
    pub const fn as_i32(&self) -> i32 {}
    #[inline]
    pub fn one_more(&self) -> usize {}
    #[inline]
    pub fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError> {}
    #[inline]
    pub fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex {}
    #[inline]
    pub fn to_ne_bytes(&self) -> [u8; 4] {}
}
