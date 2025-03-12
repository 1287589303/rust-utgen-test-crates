// Answer 0

#[test]
fn test_vacant_entry_fmt_valid() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = DummyAllocator;
    let mut table = HashTable { raw: RawTable::new(allocator) };
    let insert_slot = InsertSlot { index: 0 };
    let vacant_entry = VacantEntry {
        hash: 0,
        insert_slot,
        table: &mut table,
    };

    let mut formatter = fmt::Formatter::default();
    vacant_entry.fmt(&mut formatter);
}

#[test]
fn test_vacant_entry_fmt_debug() {
    struct DummyAllocator;

    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = DummyAllocator;
    let mut table = HashTable { raw: RawTable::new(allocator) };
    let insert_slot = InsertSlot { index: 1 };
    let vacant_entry = VacantEntry {
        hash: 1,
        insert_slot,
        table: &mut table,
    };

    let mut formatter = fmt::Formatter::default();
    vacant_entry.fmt(&mut formatter);
}

