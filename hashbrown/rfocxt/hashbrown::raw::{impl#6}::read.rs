use crate::alloc::alloc::{handle_alloc_error, Layout};
use crate::control::{BitMaskIter, Group, Tag, TagSliceExt};
use crate::scopeguard::{guard, ScopeGuard};
use crate::util::{invalid_mut, likely, unlikely};
use crate::TryReserveError;
use core::array;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::mem;
use core::ptr::NonNull;
use core::slice;
use core::{hint, ptr};
#[cfg(test)]
pub(crate) use self::alloc::AllocError;
pub(crate) use self::alloc::{do_alloc, Allocator, Global};
pub struct Bucket<T> {
    ptr: NonNull<T>,
}
impl<T> Bucket<T> {
    #[inline]
    unsafe fn from_base_index(base: NonNull<T>, index: usize) -> Self {
        let ptr = if T::IS_ZERO_SIZED {
            invalid_mut(index + 1)
        } else {
            base.as_ptr().sub(index)
        };
        Self {
            ptr: NonNull::new_unchecked(ptr),
        }
    }
    #[inline]
    unsafe fn to_base_index(&self, base: NonNull<T>) -> usize {}
    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        if T::IS_ZERO_SIZED {
            invalid_mut(mem::align_of::<T>())
        } else {
            unsafe { self.ptr.as_ptr().sub(1) }
        }
    }
    #[inline]
    fn as_non_null(&self) -> NonNull<T> {}
    #[inline]
    unsafe fn next_n(&self, offset: usize) -> Self {
        let ptr = if T::IS_ZERO_SIZED {
            invalid_mut(self.ptr.as_ptr() as usize + offset)
        } else {
            self.ptr.as_ptr().sub(offset)
        };
        Self {
            ptr: NonNull::new_unchecked(ptr),
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) unsafe fn drop(&self) {}
    #[inline]
    pub(crate) unsafe fn read(&self) -> T {
        self.as_ptr().read()
    }
    #[inline]
    pub(crate) unsafe fn write(&self, val: T) {}
    #[inline]
    pub unsafe fn as_ref<'a>(&self) -> &'a T {}
    #[inline]
    pub unsafe fn as_mut<'a>(&self) -> &'a mut T {}
}
