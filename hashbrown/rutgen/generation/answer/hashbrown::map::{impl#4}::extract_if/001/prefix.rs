// Answer 0

#[test]
fn test_extract_if_with_filled_hashmap() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, DummyAllocator> = 
        HashMap::with_capacity_and_hasher_in(8, DefaultHashBuilder::new(), DummyAllocator);

    for i in 0..8 {
        map.insert(i, i);
    }

    let _extracted: ExtractIf<i32, i32, _> = map.extract_if(|k, _v| k % 2 == 0);
}

#[test]
fn test_extract_if_with_empty_hashmap() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, DummyAllocator> = 
        HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), DummyAllocator);

    let _extracted: ExtractIf<i32, i32, _> = map.extract_if(|_k, _v| false);
}

#[test]
fn test_extract_if_with_boundary_conditions() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<i32, i32, DefaultHashBuilder, DummyAllocator> = 
        HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), DummyAllocator);

    map.insert(0, 0);
    
    let _extracted: ExtractIf<i32, i32, _> = map.extract_if(|k, _v| *k == 0);
}

