// Answer 0

#[test]
fn test_new_in_empty_hashmap() {
    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = DummyAllocator;
    let map: HashMap<&str, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new_in(alloc);
    let len = map.len();
    let capacity = map.capacity();
}

#[test]
fn test_new_in_hashmap_with_allocator() {
    struct DummyAllocator;
    unsafe impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = DummyAllocator;
    let map: HashMap<i32, i32, DefaultHashBuilder, DummyAllocator> = HashMap::new_in(alloc);
    let len = map.len();
    let capacity = map.capacity();
}

