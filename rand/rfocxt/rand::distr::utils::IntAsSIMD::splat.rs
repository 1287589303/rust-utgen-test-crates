#[cfg(feature = "simd_support")]
use core::simd::prelude::*;
#[cfg(feature = "simd_support")]
use core::simd::{LaneCount, SimdElement, SupportedLaneCount};
pub(crate) trait IntAsSIMD: Sized {
    #[inline(always)]
    fn splat(scalar: Self) -> Self {
        scalar
    }
}
