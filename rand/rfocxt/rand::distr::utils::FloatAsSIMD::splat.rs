#[cfg(feature = "simd_support")]
use core::simd::prelude::*;
#[cfg(feature = "simd_support")]
use core::simd::{LaneCount, SimdElement, SupportedLaneCount};
pub(crate) trait FloatAsSIMD: Sized {
    #[cfg(test)]
    const LEN: usize = 1;
    #[inline(always)]
    fn splat(scalar: Self) -> Self {
        scalar
    }
}
