use super::group::{
    BitMaskWord, NonZeroBitMaskWord, BITMASK_ITER_MASK, BITMASK_MASK, BITMASK_STRIDE,
};
#[derive(Clone)]
pub(crate) struct BitMaskIter(pub(crate) BitMask);
#[derive(Copy, Clone)]
pub(crate) struct BitMask(pub(crate) BitMaskWord);
impl Iterator for BitMaskIter {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        let bit = self.0.lowest_set_bit()?;
        self.0 = self.0.remove_lowest_bit();
        Some(bit)
    }
}
#[allow(clippy::use_self)]
impl BitMask {
    #[inline]
    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn invert(self) -> Self {
        BitMask(self.0 ^ BITMASK_MASK)
    }
    #[inline]
    #[must_use]
    fn remove_lowest_bit(self) -> Self {
        BitMask(self.0 & (self.0 - 1))
    }
    #[inline]
    pub(crate) fn any_bit_set(self) -> bool {}
    #[inline]
    pub(crate) fn lowest_set_bit(self) -> Option<usize> {
        if let Some(nonzero) = NonZeroBitMaskWord::new(self.0) {
            Some(Self::nonzero_trailing_zeros(nonzero))
        } else {
            None
        }
    }
    #[inline]
    pub(crate) fn trailing_zeros(self) -> usize {}
    #[inline]
    fn nonzero_trailing_zeros(nonzero: NonZeroBitMaskWord) -> usize {}
    #[inline]
    pub(crate) fn leading_zeros(self) -> usize {}
}
