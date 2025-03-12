// Answer 0

#[test]
fn test_iter_with_non_empty_inner() {
    use crate::raw::{RawTable, RawIterRange};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let allocator = TestAllocator;

    let mut raw_table: RawTable<(u32, String), TestAllocator> = RawTable::with_capacity_and_hasher(10, DefaultHasher::new());
    raw_table.insert((1, "one".to_string()));
    raw_table.insert((2, "two".to_string()));

    let raw_drain = RawDrain {
        iter: RawIter {
            iter: RawIterRange {
                start: 0,
                end: 2,
                current: 0,
            },
            items: 2,
        },
        table: raw_table.inner,
        orig_table: NonNull::dangling(),
        marker: PhantomData,
    };

    let drain = Drain {
        inner: raw_drain,
    };

    let iter = drain.iter();
}

#[test]
fn test_iter_with_multiple_items() {
    use crate::raw::{RawTable, RawIterRange};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let allocator = TestAllocator;

    let mut raw_table: RawTable<(u64, String), TestAllocator> = RawTable::with_capacity_and_hasher(10, DefaultHasher::new());
    raw_table.insert((10, "ten".to_string()));
    raw_table.insert((20, "twenty".to_string()));
    raw_table.insert((30, "thirty".to_string()));

    let raw_drain = RawDrain {
        iter: RawIter {
            iter: RawIterRange {
                start: 0,
                end: 3,
                current: 0,
            },
            items: 3,
        },
        table: raw_table.inner,
        orig_table: NonNull::dangling(),
        marker: PhantomData,
    };

    let drain = Drain {
        inner: raw_drain,
    };

    let iter = drain.iter();
}

#[test]
fn test_iter_with_boundary_conditions() {
    use crate::raw::{RawTable, RawIterRange};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let allocator = TestAllocator;

    let mut raw_table: RawTable<(i32, String), TestAllocator> = RawTable::with_capacity_and_hasher(1, DefaultHasher::new());
    raw_table.insert((0, "zero".to_string())); // Including the edge case of zero

    let raw_drain = RawDrain {
        iter: RawIter {
            iter: RawIterRange {
                start: 0,
                end: 1,
                current: 0,
            },
            items: 1,
        },
        table: raw_table.inner,
        orig_table: NonNull::dangling(),
        marker: PhantomData,
    };

    let drain = Drain {
        inner: raw_drain,
    };

    let iter = drain.iter();
}

