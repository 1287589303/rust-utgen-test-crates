use core::iter::FromIterator;
use core::mem::{self, ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};
use core::{cmp, fmt, hash, isize, slice, usize};
use alloc::{
    borrow::{Borrow, BorrowMut},
    boxed::Box, string::String, vec, vec::Vec,
};
use crate::buf::{IntoIter, UninitSlice};
use crate::bytes::Vtable;
#[allow(unused)]
use crate::loom::sync::atomic::AtomicMut;
use crate::loom::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use crate::{offset_from, Buf, BufMut, Bytes, TryGetError};
static SHARED_VTABLE: Vtable = Vtable {
    clone: shared_v_clone,
    into_vec: shared_v_to_vec,
    into_mut: shared_v_to_mut,
    is_unique: shared_v_is_unique,
    drop: shared_v_drop,
};
const _: [(); 0 - mem::align_of::<Shared>() % 2] = [];
const KIND_ARC: usize = 0b0;
const KIND_VEC: usize = 0b1;
const KIND_MASK: usize = 0b1;
const MAX_ORIGINAL_CAPACITY_WIDTH: usize = 17;
const MIN_ORIGINAL_CAPACITY_WIDTH: usize = 10;
const ORIGINAL_CAPACITY_MASK: usize = 0b11100;
const ORIGINAL_CAPACITY_OFFSET: usize = 2;
const VEC_POS_OFFSET: usize = 5;
const MAX_VEC_POS: usize = usize::MAX >> VEC_POS_OFFSET;
const NOT_VEC_POS_MASK: usize = 0b11111;
#[cfg(target_pointer_width = "64")]
const PTR_WIDTH: usize = 64;
#[cfg(target_pointer_width = "32")]
const PTR_WIDTH: usize = 32;
pub trait Buf {
    fn remaining(&self) -> usize;
    #[cfg_attr(docsrs, doc(alias = "bytes"))]
    fn chunk(&self) -> &[u8];
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize;
    fn advance(&mut self, cnt: usize);
    fn has_remaining(&self) -> bool;
    fn copy_to_slice(&mut self, dst: &mut [u8]);
    fn get_u8(&mut self) -> u8;
    fn get_i8(&mut self) -> i8;
    fn get_u16(&mut self) -> u16;
    fn get_u16_le(&mut self) -> u16;
    fn get_u16_ne(&mut self) -> u16;
    fn get_i16(&mut self) -> i16;
    fn get_i16_le(&mut self) -> i16;
    fn get_i16_ne(&mut self) -> i16;
    fn get_u32(&mut self) -> u32;
    fn get_u32_le(&mut self) -> u32;
    fn get_u32_ne(&mut self) -> u32;
    fn get_i32(&mut self) -> i32;
    fn get_i32_le(&mut self) -> i32;
    fn get_i32_ne(&mut self) -> i32;
    fn get_u64(&mut self) -> u64;
    fn get_u64_le(&mut self) -> u64;
    fn get_u64_ne(&mut self) -> u64;
    fn get_i64(&mut self) -> i64;
    fn get_i64_le(&mut self) -> i64;
    fn get_i64_ne(&mut self) -> i64;
    fn get_u128(&mut self) -> u128;
    fn get_u128_le(&mut self) -> u128;
    fn get_u128_ne(&mut self) -> u128;
    fn get_i128(&mut self) -> i128;
    fn get_i128_le(&mut self) -> i128;
    fn get_i128_ne(&mut self) -> i128;
    fn get_uint(&mut self, nbytes: usize) -> u64;
    fn get_uint_le(&mut self, nbytes: usize) -> u64;
    fn get_uint_ne(&mut self, nbytes: usize) -> u64;
    fn get_int(&mut self, nbytes: usize) -> i64;
    fn get_int_le(&mut self, nbytes: usize) -> i64;
    fn get_int_ne(&mut self, nbytes: usize) -> i64;
    fn get_f32(&mut self) -> f32;
    fn get_f32_le(&mut self) -> f32;
    fn get_f32_ne(&mut self) -> f32;
    fn get_f64(&mut self) -> f64;
    fn get_f64_le(&mut self) -> f64;
    fn get_f64_ne(&mut self) -> f64;
    fn try_copy_to_slice(&mut self, mut dst: &mut [u8]) -> Result<(), TryGetError>;
    fn try_get_u8(&mut self) -> Result<u8, TryGetError>;
    fn try_get_i8(&mut self) -> Result<i8, TryGetError>;
    fn try_get_u16(&mut self) -> Result<u16, TryGetError>;
    fn try_get_u16_le(&mut self) -> Result<u16, TryGetError>;
    fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError>;
    fn try_get_i16(&mut self) -> Result<i16, TryGetError>;
    fn try_get_i16_le(&mut self) -> Result<i16, TryGetError>;
    fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError>;
    fn try_get_u32(&mut self) -> Result<u32, TryGetError>;
    fn try_get_u32_le(&mut self) -> Result<u32, TryGetError>;
    fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError>;
    fn try_get_i32(&mut self) -> Result<i32, TryGetError>;
    fn try_get_i32_le(&mut self) -> Result<i32, TryGetError>;
    fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError>;
    fn try_get_u64(&mut self) -> Result<u64, TryGetError>;
    fn try_get_u64_le(&mut self) -> Result<u64, TryGetError>;
    fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError>;
    fn try_get_i64(&mut self) -> Result<i64, TryGetError>;
    fn try_get_i64_le(&mut self) -> Result<i64, TryGetError>;
    fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError>;
    fn try_get_u128(&mut self) -> Result<u128, TryGetError>;
    fn try_get_u128_le(&mut self) -> Result<u128, TryGetError>;
    fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError>;
    fn try_get_i128(&mut self) -> Result<i128, TryGetError>;
    fn try_get_i128_le(&mut self) -> Result<i128, TryGetError>;
    fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError>;
    fn try_get_uint(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_uint_le(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_uint_ne(&mut self, nbytes: usize) -> Result<u64, TryGetError>;
    fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_int_le(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_int_ne(&mut self, nbytes: usize) -> Result<i64, TryGetError>;
    fn try_get_f32(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f32_le(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError>;
    fn try_get_f64(&mut self) -> Result<f64, TryGetError>;
    fn try_get_f64_le(&mut self) -> Result<f64, TryGetError>;
    fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError>;
    fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes;
    fn take(self, limit: usize) -> Take<Self>
    where
        Self: Sized,
    {
        take::new(self, limit)
    }
    fn chain<U: Buf>(self, next: U) -> Chain<Self, U>
    where
        Self: Sized,
    {
        Chain::new(self, next)
    }
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn reader(self) -> Reader<Self>
    where
        Self: Sized,
    {
        reader::new(self)
    }
}
pub unsafe trait BufMut {
    fn remaining_mut(&self) -> usize;
    unsafe fn advance_mut(&mut self, cnt: usize);
    #[inline]
    fn has_remaining_mut(&self) -> bool;
    #[cfg_attr(docsrs, doc(alias = "bytes_mut"))]
    fn chunk_mut(&mut self) -> &mut UninitSlice;
    #[inline]
    fn put<T: super::Buf>(&mut self, mut src: T)
    where
        Self: Sized;
    #[inline]
    fn put_slice(&mut self, mut src: &[u8]);
    #[inline]
    fn put_bytes(&mut self, val: u8, mut cnt: usize);
    #[inline]
    fn put_u8(&mut self, n: u8);
    #[inline]
    fn put_i8(&mut self, n: i8);
    #[inline]
    fn put_u16(&mut self, n: u16);
    #[inline]
    fn put_u16_le(&mut self, n: u16);
    #[inline]
    fn put_u16_ne(&mut self, n: u16);
    #[inline]
    fn put_i16(&mut self, n: i16);
    #[inline]
    fn put_i16_le(&mut self, n: i16);
    #[inline]
    fn put_i16_ne(&mut self, n: i16);
    #[inline]
    fn put_u32(&mut self, n: u32);
    #[inline]
    fn put_u32_le(&mut self, n: u32);
    #[inline]
    fn put_u32_ne(&mut self, n: u32);
    #[inline]
    fn put_i32(&mut self, n: i32);
    #[inline]
    fn put_i32_le(&mut self, n: i32);
    #[inline]
    fn put_i32_ne(&mut self, n: i32);
    #[inline]
    fn put_u64(&mut self, n: u64);
    #[inline]
    fn put_u64_le(&mut self, n: u64);
    #[inline]
    fn put_u64_ne(&mut self, n: u64);
    #[inline]
    fn put_i64(&mut self, n: i64);
    #[inline]
    fn put_i64_le(&mut self, n: i64);
    #[inline]
    fn put_i64_ne(&mut self, n: i64);
    #[inline]
    fn put_u128(&mut self, n: u128);
    #[inline]
    fn put_u128_le(&mut self, n: u128);
    #[inline]
    fn put_u128_ne(&mut self, n: u128);
    #[inline]
    fn put_i128(&mut self, n: i128);
    #[inline]
    fn put_i128_le(&mut self, n: i128);
    #[inline]
    fn put_i128_ne(&mut self, n: i128);
    #[inline]
    fn put_uint(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_uint_le(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_uint_ne(&mut self, n: u64, nbytes: usize);
    #[inline]
    fn put_int(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_int_le(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_int_ne(&mut self, n: i64, nbytes: usize);
    #[inline]
    fn put_f32(&mut self, n: f32);
    #[inline]
    fn put_f32_le(&mut self, n: f32);
    #[inline]
    fn put_f32_ne(&mut self, n: f32);
    #[inline]
    fn put_f64(&mut self, n: f64);
    #[inline]
    fn put_f64_le(&mut self, n: f64);
    #[inline]
    fn put_f64_ne(&mut self, n: f64);
    #[inline]
    fn limit(self, limit: usize) -> Limit<Self>
    where
        Self: Sized,
    {
        limit::new(self, limit)
    }
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[inline]
    fn writer(self) -> Writer<Self>
    where
        Self: Sized,
    {
        writer::new(self)
    }
    #[inline]
    fn chain_mut<U: BufMut>(self, next: U) -> Chain<Self, U>
    where
        Self: Sized,
    {
        Chain::new(self, next)
    }
}
pub struct Bytes {
    ptr: *const u8,
    len: usize,
    data: AtomicPtr<()>,
    vtable: &'static Vtable,
}
impl<'a> IntoIterator for &'a BytesMut {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}
impl Bytes {
    #[inline]
    #[cfg(not(all(loom, test)))]
    pub const fn new() -> Self {
        const EMPTY: &[u8] = &[];
        Bytes::from_static(EMPTY)
    }
    #[cfg(all(loom, test))]
    pub fn new() -> Self {
        const EMPTY: &[u8] = &[];
        Bytes::from_static(EMPTY)
    }
    #[inline]
    #[cfg(not(all(loom, test)))]
    pub const fn from_static(bytes: &'static [u8]) -> Self {
        Bytes {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
            data: AtomicPtr::new(ptr::null_mut()),
            vtable: &STATIC_VTABLE,
        }
    }
    #[cfg(all(loom, test))]
    pub fn from_static(bytes: &'static [u8]) -> Self {
        Bytes {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
            data: AtomicPtr::new(ptr::null_mut()),
            vtable: &STATIC_VTABLE,
        }
    }
    fn new_empty_with_ptr(ptr: *const u8) -> Self {
        debug_assert!(! ptr.is_null());
        let ptr = without_provenance(ptr as usize);
        Bytes {
            ptr,
            len: 0,
            data: AtomicPtr::new(ptr::null_mut()),
            vtable: &STATIC_VTABLE,
        }
    }
    pub fn from_owner<T>(owner: T) -> Self
    where
        T: AsRef<[u8]> + Send + 'static,
    {
        let owned = Box::into_raw(
            Box::new(Owned {
                lifetime: OwnedLifetime {
                    ref_cnt: AtomicUsize::new(1),
                    drop: owned_box_and_drop::<T>,
                },
                owner,
            }),
        );
        let mut ret = Bytes {
            ptr: NonNull::dangling().as_ptr(),
            len: 0,
            data: AtomicPtr::new(owned.cast()),
            vtable: &OWNED_VTABLE,
        };
        let buf = unsafe { &*owned }.owner.as_ref();
        ret.ptr = buf.as_ptr();
        ret.len = buf.len();
        ret
    }
    #[inline]
    pub const fn len(&self) -> usize {}
    #[inline]
    pub const fn is_empty(&self) -> bool {}
    pub fn is_unique(&self) -> bool {}
    pub fn copy_from_slice(data: &[u8]) -> Self {
        data.to_vec().into()
    }
    pub fn slice(&self, range: impl RangeBounds<usize>) -> Self {
        use core::ops::Bound;
        let len = self.len();
        let begin = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n.checked_add(1).expect("out of range"),
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n.checked_add(1).expect("out of range"),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };
        assert!(
            begin <= end, "range start must not be greater than end: {:?} <= {:?}",
            begin, end,
        );
        assert!(end <= len, "range end out of bounds: {:?} <= {:?}", end, len,);
        if end == begin {
            return Bytes::new();
        }
        let mut ret = self.clone();
        ret.len = end - begin;
        ret.ptr = unsafe { ret.ptr.add(begin) };
        ret
    }
    pub fn slice_ref(&self, subset: &[u8]) -> Self {
        if subset.is_empty() {
            return Bytes::new();
        }
        let bytes_p = self.as_ptr() as usize;
        let bytes_len = self.len();
        let sub_p = subset.as_ptr() as usize;
        let sub_len = subset.len();
        assert!(
            sub_p >= bytes_p,
            "subset pointer ({:p}) is smaller than self pointer ({:p})", subset.as_ptr(),
            self.as_ptr(),
        );
        assert!(
            sub_p + sub_len <= bytes_p + bytes_len,
            "subset is out of bounds: self = ({:p}, {}), subset = ({:p}, {})", self
            .as_ptr(), bytes_len, subset.as_ptr(), sub_len,
        );
        let sub_offset = sub_p - bytes_p;
        self.slice(sub_offset..(sub_offset + sub_len))
    }
    #[must_use = "consider Bytes::truncate if you don't need the other half"]
    pub fn split_off(&mut self, at: usize) -> Self {
        if at == self.len() {
            return Bytes::new_empty_with_ptr(self.ptr.wrapping_add(at));
        }
        if at == 0 {
            return mem::replace(self, Bytes::new_empty_with_ptr(self.ptr));
        }
        assert!(
            at <= self.len(), "split_off out of bounds: {:?} <= {:?}", at, self.len(),
        );
        let mut ret = self.clone();
        self.len = at;
        unsafe { ret.inc_start(at) };
        ret
    }
    #[must_use = "consider Bytes::advance if you don't need the other half"]
    pub fn split_to(&mut self, at: usize) -> Self {
        if at == self.len() {
            let end_ptr = self.ptr.wrapping_add(at);
            return mem::replace(self, Bytes::new_empty_with_ptr(end_ptr));
        }
        if at == 0 {
            return Bytes::new_empty_with_ptr(self.ptr);
        }
        assert!(
            at <= self.len(), "split_to out of bounds: {:?} <= {:?}", at, self.len(),
        );
        let mut ret = self.clone();
        unsafe { self.inc_start(at) };
        ret.len = at;
        ret
    }
    #[inline]
    pub fn truncate(&mut self, len: usize) {}
    #[inline]
    pub fn clear(&mut self) {}
    pub fn try_into_mut(self) -> Result<BytesMut, Bytes> {}
    #[inline]
    pub(crate) unsafe fn with_vtable(
        ptr: *const u8,
        len: usize,
        data: AtomicPtr<()>,
        vtable: &'static Vtable,
    ) -> Bytes {}
    #[inline]
    fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
    #[inline]
    unsafe fn inc_start(&mut self, by: usize) {}
}
