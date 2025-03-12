// Answer 0

#[test]
fn test_insert_entry_with_zero_hash() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0)) as *mut u8))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            drop(Box::from_raw(ptr.as_ptr() as *mut i32));
        }
    }

    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(TestAllocator);
    let hash = 0u64;
    let value = 42;
    let hasher = |val: &i32| *val as u64;

    unsafe {
        let entry = table.insert_entry(hash, value, hasher);
    }
}

#[test]
fn test_insert_entry_with_max_hash() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0)) as *mut u8))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            drop(Box::from_raw(ptr.as_ptr() as *mut i32));
        }
    }

    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(TestAllocator);
    let hash = u64::MAX;
    let value = 100;
    let hasher = |val: &i32| *val as u64;

    unsafe {
        let entry = table.insert_entry(hash, value, hasher);
    }
}

#[test]
fn test_insert_entry_with_negative_value() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0)) as *mut u8))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            drop(Box::from_raw(ptr.as_ptr() as *mut i32));
        }
    }

    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(TestAllocator);
    let hash = 1u64;
    let value = -5;
    let hasher = |val: &i32| *val as u64;

    unsafe {
        let entry = table.insert_entry(hash, value, hasher);
    }
}

#[test]
fn test_insert_entry_with_large_positive_value() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0)) as *mut u8))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            drop(Box::from_raw(ptr.as_ptr() as *mut i32));
        }
    }

    let mut table: RawTable<i32, TestAllocator> = RawTable::new_in(TestAllocator);
    let hash = 2u64;
    let value = 10000;
    let hasher = |val: &i32| *val as u64;

    unsafe {
        let entry = table.insert_entry(hash, value, hasher);
    }
}

#[test]
fn test_insert_entry_with_custom_struct() {
    struct CustomType {
        id: usize,
        name: String,
    }

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0)) as *mut u8))
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
            drop(Box::from_raw(ptr.as_ptr() as *mut CustomType));
        }
    }

    let mut table: RawTable<CustomType, TestAllocator> = RawTable::new_in(TestAllocator);
    let hash = 3u64;
    let value = CustomType { id: 1, name: "Test".to_string() };
    let hasher = |val: &CustomType| val.id as u64;

    unsafe {
        let entry = table.insert_entry(hash, value, hasher);
    }
}

