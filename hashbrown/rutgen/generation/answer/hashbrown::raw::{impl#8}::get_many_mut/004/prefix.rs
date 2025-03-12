// Answer 0

#[test]
fn test_get_many_mut_one_entry() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    table.insert(1, 10, |v: &u32| *v);
    
    let result = table.get_many_mut([1], |i, k| *k == i as u32);
}

#[test]
fn test_get_many_mut_two_entries() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    table.insert(1, 10, |v: &u32| *v);
    table.insert(2, 20, |v: &u32| *v);
    
    let result = table.get_many_mut([1, 2], |i, k| *k == i as u32);
}

#[test]
fn test_get_many_mut_three_entries_no_duplicates() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    table.insert(3, 30, |v: &u32| *v);
    table.insert(4, 40, |v: &u32| *v);
    table.insert(5, 50, |v: &u32| *v);
    
    let result = table.get_many_mut([3, 4, 5], |i, k| *k == i as u32);
}

#[test]
fn test_get_many_mut_boundary_case_unique_hashes() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    table.insert(0, 100, |v: &u32| *v);
    table.insert(u64::MAX, 200, |v: &u32| *v);
    
    let result = table.get_many_mut([0, u64::MAX], |i, k| *k == i as u32);
} 

#[test]
#[should_panic] 
fn test_get_many_mut_duplicate_hashes() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::new_in(TestAllocator);
    table.insert(6, 60, |v: &u32| *v);
    
    let result = table.get_many_mut([6, 6], |i, k| *k == i as u32);
}

