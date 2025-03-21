// Answer 0

#[test]
fn test_search_returns_none_when_no_match() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let map: HashMap<i32, i32, core::hash::BuildHasherDefault<core::hash::Hasher>> = HashMap {
        hash_builder: core::hash::BuildHasherDefault::default(),
        table: RawTable {
            table: RawTableInner::default(),
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let builder = RawEntryBuilder { map: &map };
    
    let hash: u64 = 123456789; // Some valid u64 value
    let is_match = |_: &i32| false; // Always returns false

    let result = builder.search(hash, is_match);
}

