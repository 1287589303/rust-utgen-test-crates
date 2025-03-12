use super::group::{
    BitMaskWord, NonZeroBitMaskWord, BITMASK_ITER_MASK, BITMASK_MASK, BITMASK_STRIDE,
};
#[derive(Copy, Clone)]
pub(crate) struct BitMask(pub(crate) BitMaskWord);
#[derive(Clone)]
pub(crate) struct BitMaskIter(pub(crate) BitMask);
impl IntoIterator for BitMask {
    type Item = usize;
    type IntoIter = BitMaskIter;
    #[inline]
    fn into_iter(self) -> BitMaskIter {
        BitMaskIter(BitMask(self.0 & BITMASK_ITER_MASK))
    }
}
