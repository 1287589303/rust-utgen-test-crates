// Answer 0

#[test]
fn test_absent_entry_debug_fmt() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let raw_table = RawTable::<i32, TestAllocator>::new();
    let mut hash_table = HashTable {
        raw: raw_table,
    };

    let absent_entry = AbsentEntry {
        table: &mut hash_table,
    };

    let mut formatter = std::fmt::Formatter::new();
    let _ = absent_entry.fmt(&mut formatter);
}

#[test]
fn test_absent_entry_debug_fmt_with_different_allocator() {
    struct AnotherAllocator;

    unsafe impl Allocator for AnotherAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = AnotherAllocator;
    let raw_table = RawTable::<String, AnotherAllocator>::new();
    let mut hash_table = HashTable {
        raw: raw_table,
    };

    let absent_entry = AbsentEntry {
        table: &mut hash_table,
    };

    let mut formatter = std::fmt::Formatter::new();
    let _ = absent_entry.fmt(&mut formatter);
}

