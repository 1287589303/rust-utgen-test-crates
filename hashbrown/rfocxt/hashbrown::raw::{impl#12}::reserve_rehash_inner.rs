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
#[derive(Copy, Clone)]
struct TableLayout {
    size: usize,
    ctrl_align: usize,
}
#[derive(Copy, Clone)]
enum Fallibility {
    Fallible,
    Infallible,
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TryReserveError {
    /// Error due to the computed capacity exceeding the collection's maximum
    /// (usually `isize::MAX` bytes).
    CapacityOverflow,
    /// The memory allocator returned an error
    AllocError {
        /// The layout of the allocation request that failed.
        layout: alloc::alloc::Layout,
    },
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
    unsafe fn find_insert_slot(&self, hash: u64) -> InsertSlot {}
    #[inline(always)]
    unsafe fn find_inner(
        &self,
        hash: u64,
        eq: &mut dyn FnMut(usize) -> bool,
    ) -> Option<usize> {}
    #[allow(clippy::mut_mut)]
    #[inline]
    unsafe fn prepare_rehash_in_place(&mut self) {}
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
    unsafe fn bucket_ptr(&self, index: usize, size_of: usize) -> *mut u8 {}
    #[inline]
    fn data_end<T>(&self) -> NonNull<T> {}
    #[inline]
    fn probe_seq(&self, hash: u64) -> ProbeSeq {}
    #[inline]
    unsafe fn record_item_insert_at(&mut self, index: usize, old_ctrl: Tag, hash: u64) {}
    #[inline]
    fn is_in_same_group(&self, i: usize, new_i: usize, hash: u64) -> bool {}
    #[inline]
    unsafe fn set_ctrl_hash(&mut self, index: usize, hash: u64) {}
    #[inline]
    unsafe fn replace_ctrl_hash(&mut self, index: usize, hash: u64) -> Tag {}
    #[inline]
    unsafe fn set_ctrl(&mut self, index: usize, ctrl: Tag) {}
    #[inline]
    unsafe fn ctrl(&self, index: usize) -> *mut Tag {}
    fn ctrl_slice(&mut self) -> &mut [Tag] {}
    #[inline]
    fn buckets(&self) -> usize {}
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
    {
        let new_items = match self.items.checked_add(additional) {
            Some(new_items) => new_items,
            None => return Err(fallibility.capacity_overflow()),
        };
        let full_capacity = bucket_mask_to_capacity(self.bucket_mask);
        if new_items <= full_capacity / 2 {
            self.rehash_in_place(hasher, layout.size, drop);
            Ok(())
        } else {
            self.resize_inner(
                alloc,
                usize::max(new_items, full_capacity + 1),
                hasher,
                fallibility,
                layout,
            )
        }
    }
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
    {
        let mut new_table = self.prepare_resize(alloc, layout, capacity, fallibility)?;
        for full_byte_index in self.full_buckets_indices() {
            let hash = hasher(self, full_byte_index);
            let (new_index, _) = new_table.prepare_insert_slot(hash);
            ptr::copy_nonoverlapping(
                self.bucket_ptr(full_byte_index, layout.size),
                new_table.bucket_ptr(new_index, layout.size),
                layout.size,
            );
        }
        new_table.growth_left -= self.items;
        new_table.items = self.items;
        mem::swap(self, &mut new_table);
        Ok(())
    }
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
impl Fallibility {
    #[cfg_attr(feature = "inline-more", inline)]
    fn capacity_overflow(self) -> TryReserveError {
        match self {
            Fallibility::Fallible => TryReserveError::CapacityOverflow,
            Fallibility::Infallible => panic!("Hash table capacity overflow"),
        }
    }
    #[cfg_attr(feature = "inline-more", inline)]
    fn alloc_err(self, layout: Layout) -> TryReserveError {}
}
#[inline]
fn bucket_mask_to_capacity(bucket_mask: usize) -> usize {
    if bucket_mask < 8 { bucket_mask } else { ((bucket_mask + 1) / 8) * 7 }
}
