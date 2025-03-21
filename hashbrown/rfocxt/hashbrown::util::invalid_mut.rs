#[cfg(not(feature = "nightly"))]
pub(crate) use core::convert::{identity as likely, identity as unlikely};
#[cfg(feature = "nightly")]
pub(crate) use core::intrinsics::{likely, unlikely};
#[inline(always)]
#[allow(clippy::useless_transmute)]
pub(crate) fn invalid_mut<T>(addr: usize) -> *mut T {
    unsafe { core::mem::transmute(addr) }
}
