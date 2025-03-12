use core::iter::FromIterator;
use core::mem::{self, ManuallyDrop};
use core::ops::{Deref, RangeBounds};
use core::ptr::NonNull;
use core::{cmp, fmt, hash, ptr, slice, usize};
use alloc::{
    alloc::{dealloc, Layout},
    borrow::Borrow, boxed::Box, string::String, vec::Vec,
};
use crate::buf::IntoIter;
#[allow(unused)]
use crate::loom::sync::atomic::AtomicMut;
use crate::loom::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use crate::{offset_from, Buf, BytesMut};
static OWNED_VTABLE: Vtable = Vtable {
    clone: owned_clone,
    into_vec: owned_to_vec,
    into_mut: owned_to_mut,
    is_unique: owned_is_unique,
    drop: owned_drop,
};
static PROMOTABLE_EVEN_VTABLE: Vtable = Vtable {
    clone: promotable_even_clone,
    into_vec: promotable_even_to_vec,
    into_mut: promotable_even_to_mut,
    is_unique: promotable_is_unique,
    drop: promotable_even_drop,
};
static PROMOTABLE_ODD_VTABLE: Vtable = Vtable {
    clone: promotable_odd_clone,
    into_vec: promotable_odd_to_vec,
    into_mut: promotable_odd_to_mut,
    is_unique: promotable_is_unique,
    drop: promotable_odd_drop,
};
static SHARED_VTABLE: Vtable = Vtable {
    clone: shared_clone,
    into_vec: shared_to_vec,
    into_mut: shared_to_mut,
    is_unique: shared_is_unique,
    drop: shared_drop,
};
const STATIC_VTABLE: Vtable = Vtable {
    clone: static_clone,
    into_vec: static_to_vec,
    into_mut: static_to_mut,
    is_unique: static_is_unique,
    drop: static_drop,
};
const _: [(); 0 - mem::align_of::<Shared>() % 2] = [];
const KIND_ARC: usize = 0b0;
const KIND_VEC: usize = 0b1;
const KIND_MASK: usize = 0b1;
pub(crate) struct Vtable {
    /// fn(data, ptr, len)
    pub clone: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes,
    /// fn(data, ptr, len)
    ///
    /// `into_*` consumes the `Bytes`, returning the respective value.
    pub into_vec: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Vec<u8>,
    pub into_mut: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> BytesMut,
    /// fn(data)
    pub is_unique: unsafe fn(&AtomicPtr<()>) -> bool,
    /// fn(data, ptr, len)
    pub drop: unsafe fn(&mut AtomicPtr<()>, *const u8, usize),
}
pub struct Bytes {
    ptr: *const u8,
    len: usize,
    data: AtomicPtr<()>,
    vtable: &'static Vtable,
}
struct Shared {
    buf: *mut u8,
    cap: usize,
    ref_cnt: AtomicUsize,
}
unsafe fn shallow_clone_arc(shared: *mut Shared, ptr: *const u8, len: usize) -> Bytes {
    let old_size = (*shared).ref_cnt.fetch_add(1, Ordering::Relaxed);
    if old_size > usize::MAX >> 1 {
        crate::abort();
    }
    Bytes {
        ptr,
        len,
        data: AtomicPtr::new(shared as _),
        vtable: &SHARED_VTABLE,
    }
}
#[inline(never)]
#[cold]
fn abort() -> ! {
    #[cfg(feature = "std")]
    {
        std::process::abort();
    }
    #[cfg(not(feature = "std"))]
    {
        struct Abort;
        impl Drop for Abort {
            fn drop(&mut self) {
                panic!();
            }
        }
        let _a = Abort;
        panic!("abort");
    }
}
