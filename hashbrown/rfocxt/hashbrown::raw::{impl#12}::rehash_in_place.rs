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
struct RawTableInner {
    bucket_mask: usize,
    ctrl: NonNull<u8>,
    growth_left: usize,
    items: usize,
}
pub struct ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    dropfn: F,
    value: T,
}
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct Tag(pub(super) u8);
pub struct InsertSlot {
    index: usize,
}
impl RawTableInner {
    #[cfg_attr(feature = "inline-more", inline)]
    unsafe fn new_uninitialized<A>(
        alloc: &A,
        table_layout: TableLayout,
        buckets: usize,
        fallibility: Fallibility,
    ) -> Result<Self, TryReserveError>
    where
        A: Allocator,
    {
        debug_assert!(buckets.is_power_of_two());
        let (layout, ctrl_offset) = match table_layout.calculate_layout_for(buckets) {
            Some(lco) => lco,
            None => return Err(fallibility.capacity_overflow()),
        };
        let ptr: NonNull<u8> = match do_alloc(alloc, layout) {
            Ok(block) => block.cast(),
            Err(_) => return Err(fallibility.alloc_err(layout)),
        };
        let ctrl = NonNull::new_unchecked(ptr.as_ptr().add(ctrl_offset));
        Ok(Self {
            ctrl,
            bucket_mask: buckets - 1,
            items: 0,
            growth_left: bucket_mask_to_capacity(buckets - 1),
        })
    }
    #[inline]
    fn fallible_with_capacity<A>(
        alloc: &A,
        table_layout: TableLayout,
        capacity: usize,
        fallibility: Fallibility,
    ) -> Result<Self, TryReserveError>
    where
        A: Allocator,
    {
        if capacity == 0 {
            Ok(Self::NEW)
        } else {
            unsafe {
                let buckets = capacity_to_buckets(capacity)
                    .ok_or_else(|| fallibility.capacity_overflow())?;
                let mut result = Self::new_uninitialized(
                    alloc,
                    table_layout,
                    buckets,
                    fallibility,
                )?;
                result.ctrl_slice().fill_empty();
                Ok(result)
            }
        }
    }
    fn with_capacity<A>(alloc: &A, table_layout: TableLayout, capacity: usize) -> Self
    where
        A: Allocator,
    {
        match Self::fallible_with_capacity(
            alloc,
            table_layout,
            capacity,
            Fallibility::Infallible,
        ) {
            Ok(table_inner) => table_inner,
            Err(_) => unsafe { hint::unreachable_unchecked() }
        }
    }
    #[inline]
    unsafe fn fix_insert_slot(&self, mut index: usize) -> InsertSlot {}
    #[inline]
    fn find_insert_slot_in_group(
        &self,
        group: &Group,
        probe_seq: &ProbeSeq,
    ) -> Option<usize> {}
    #[inline]
    unsafe fn find_or_find_insert_slot_inner(
        &self,
        hash: u64,
        eq: &mut dyn FnMut(usize) -> bool,
    ) -> Result<usize, InsertSlot> {}
    #[inline]
    unsafe fn prepare_insert_slot(&mut self, hash: u64) -> (usize, Tag) {}
    #[inline]
    unsafe fn find_insert_slot(&self, hash: u64) -> InsertSlot {
        let mut probe_seq = self.probe_seq(hash);
        loop {
            let group = unsafe { Group::load(self.ctrl(probe_seq.pos)) };
            let index = self.find_insert_slot_in_group(&group, &probe_seq);
            if likely(index.is_some()) {
                unsafe {
                    return self.fix_insert_slot(index.unwrap_unchecked());
                }
            }
            probe_seq.move_next(self.bucket_mask);
        }
    }
    #[inline(always)]
    unsafe fn find_inner(
        &self,
        hash: u64,
        eq: &mut dyn FnMut(usize) -> bool,
    ) -> Option<usize> {}
    #[allow(clippy::mut_mut)]
    #[inline]
    unsafe fn prepare_rehash_in_place(&mut self) {
        for i in (0..self.buckets()).step_by(Group::WIDTH) {
            let group = Group::load_aligned(self.ctrl(i));
            let group = group.convert_special_to_empty_and_full_to_deleted();
            group.store_aligned(self.ctrl(i));
        }
        if unlikely(self.buckets() < Group::WIDTH) {
            self.ctrl(0).copy_to(self.ctrl(Group::WIDTH), self.buckets());
        } else {
            self.ctrl(0).copy_to(self.ctrl(self.buckets()), Group::WIDTH);
        }
    }
    #[inline]
    unsafe fn iter<T>(&self) -> RawIter<T> {}
    unsafe fn drop_elements<T>(&mut self) {}
    unsafe fn drop_inner_table<T, A: Allocator>(
        &mut self,
        alloc: &A,
        table_layout: TableLayout,
    ) {}
    #[inline]
    unsafe fn bucket<T>(&self, index: usize) -> Bucket<T> {}
    #[inline]
    unsafe fn bucket_ptr(&self, index: usize, size_of: usize) -> *mut u8 {
        debug_assert_ne!(self.bucket_mask, 0);
        debug_assert!(index < self.buckets());
        let base: *mut u8 = self.data_end().as_ptr();
        base.sub((index + 1) * size_of)
    }
    #[inline]
    fn data_end<T>(&self) -> NonNull<T> {}
    #[inline]
    fn probe_seq(&self, hash: u64) -> ProbeSeq {}
    #[inline]
    unsafe fn record_item_insert_at(&mut self, index: usize, old_ctrl: Tag, hash: u64) {}
    #[inline]
    fn is_in_same_group(&self, i: usize, new_i: usize, hash: u64) -> bool {
        let probe_seq_pos = self.probe_seq(hash).pos;
        let probe_index = |pos: usize| {
            (pos.wrapping_sub(probe_seq_pos) & self.bucket_mask) / Group::WIDTH
        };
        probe_index(i) == probe_index(new_i)
    }
    #[inline]
    unsafe fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
        self.set_ctrl(index, Tag::full(hash));
    }
    #[inline]
    unsafe fn replace_ctrl_hash(&mut self, index: usize, hash: u64) -> Tag {
        let prev_ctrl = *self.ctrl(index);
        self.set_ctrl_hash(index, hash);
        prev_ctrl
    }
    #[inline]
    unsafe fn set_ctrl(&mut self, index: usize, ctrl: Tag) {
        let index2 = ((index.wrapping_sub(Group::WIDTH)) & self.bucket_mask)
            + Group::WIDTH;
        *self.ctrl(index) = ctrl;
        *self.ctrl(index2) = ctrl;
    }
    #[inline]
    unsafe fn ctrl(&self, index: usize) -> *mut Tag {
        debug_assert!(index < self.num_ctrl_bytes());
        self.ctrl.as_ptr().add(index).cast()
    }
    fn ctrl_slice(&mut self) -> &mut [Tag] {}
    #[inline]
    fn buckets(&self) -> usize {
        self.bucket_mask + 1
    }
    #[inline]
    unsafe fn is_bucket_full(&self, index: usize) -> bool {}
    #[inline]
    fn num_ctrl_bytes(&self) -> usize {}
    #[inline]
    fn is_empty_singleton(&self) -> bool {}
    #[allow(clippy::mut_mut)]
    #[inline]
    fn prepare_resize<'a, A>(
        &self,
        alloc: &'a A,
        table_layout: TableLayout,
        capacity: usize,
        fallibility: Fallibility,
    ) -> Result<
        crate::scopeguard::ScopeGuard<Self, impl FnMut(&mut Self) + 'a>,
        TryReserveError,
    >
    where
        A: Allocator,
    {
        debug_assert!(self.items <= capacity);
        let new_table = RawTableInner::fallible_with_capacity(
            alloc,
            table_layout,
            capacity,
            fallibility,
        )?;
        Ok(
            guard(
                new_table,
                move |self_| {
                    if !self_.is_empty_singleton() {
                        unsafe { self_.free_buckets(alloc, table_layout) };
                    }
                },
            ),
        )
    }
    #[allow(clippy::inline_always)]
    #[inline(always)]
    unsafe fn reserve_rehash_inner<A>(
        &mut self,
        alloc: &A,
        additional: usize,
        hasher: &dyn Fn(&mut Self, usize) -> u64,
        fallibility: Fallibility,
        layout: TableLayout,
        drop: Option<unsafe fn(*mut u8)>,
    ) -> Result<(), TryReserveError>
    where
        A: Allocator,
    {}
    #[inline(always)]
    unsafe fn full_buckets_indices(&self) -> FullBucketsIndices {}
    #[allow(clippy::inline_always)]
    #[inline(always)]
    unsafe fn resize_inner<A>(
        &mut self,
        alloc: &A,
        capacity: usize,
        hasher: &dyn Fn(&mut Self, usize) -> u64,
        fallibility: Fallibility,
        layout: TableLayout,
    ) -> Result<(), TryReserveError>
    where
        A: Allocator,
    {}
    #[allow(clippy::inline_always)]
    #[cfg_attr(feature = "inline-more", inline(always))]
    #[cfg_attr(not(feature = "inline-more"), inline)]
    unsafe fn rehash_in_place(
        &mut self,
        hasher: &dyn Fn(&mut Self, usize) -> u64,
        size_of: usize,
        drop: Option<unsafe fn(*mut u8)>,
    ) {
        self.prepare_rehash_in_place();
        let mut guard = guard(
            self,
            move |self_| {
                if let Some(drop) = drop {
                    for i in 0..self_.buckets() {
                        if *self_.ctrl(i) == Tag::DELETED {
                            self_.set_ctrl(i, Tag::EMPTY);
                            drop(self_.bucket_ptr(i, size_of));
                            self_.items -= 1;
                        }
                    }
                }
                self_.growth_left = bucket_mask_to_capacity(self_.bucket_mask)
                    - self_.items;
            },
        );
        'outer: for i in 0..guard.buckets() {
            if *guard.ctrl(i) != Tag::DELETED {
                continue;
            }
            let i_p = guard.bucket_ptr(i, size_of);
            'inner: loop {
                let hash = hasher(*guard, i);
                let new_i = guard.find_insert_slot(hash).index;
                if likely(guard.is_in_same_group(i, new_i, hash)) {
                    guard.set_ctrl_hash(i, hash);
                    continue 'outer;
                }
                let new_i_p = guard.bucket_ptr(new_i, size_of);
                let prev_ctrl = guard.replace_ctrl_hash(new_i, hash);
                if prev_ctrl == Tag::EMPTY {
                    guard.set_ctrl(i, Tag::EMPTY);
                    ptr::copy_nonoverlapping(i_p, new_i_p, size_of);
                    continue 'outer;
                } else {
                    debug_assert_eq!(prev_ctrl, Tag::DELETED);
                    ptr::swap_nonoverlapping(i_p, new_i_p, size_of);
                    continue 'inner;
                }
            }
        }
        guard.growth_left = bucket_mask_to_capacity(guard.bucket_mask) - guard.items;
        mem::forget(guard);
    }
    #[inline]
    unsafe fn free_buckets<A>(&mut self, alloc: &A, table_layout: TableLayout)
    where
        A: Allocator,
    {}
    #[inline]
    unsafe fn allocation_info(
        &self,
        table_layout: TableLayout,
    ) -> (NonNull<u8>, Layout) {}
    #[inline]
    unsafe fn allocation_size_or_zero(&self, table_layout: TableLayout) -> usize {}
    #[inline]
    fn clear_no_drop(&mut self) {}
    #[inline]
    unsafe fn erase(&mut self, index: usize) {}
}
#[inline]
fn bucket_mask_to_capacity(bucket_mask: usize) -> usize {
    if bucket_mask < 8 { bucket_mask } else { ((bucket_mask + 1) / 8) * 7 }
}
#[inline]
pub fn guard<T, F>(value: T, dropfn: F) -> ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    ScopeGuard { dropfn, value }
}
