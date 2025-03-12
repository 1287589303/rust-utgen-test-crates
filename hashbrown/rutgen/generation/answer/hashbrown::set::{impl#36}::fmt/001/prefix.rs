// Answer 0

#[test]
fn test_into_iter_debug_non_empty() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = MockAllocator;
    let mut map: HashMap<i32, i32, _> = HashMap::with_hasher(DefaultHashBuilder::new());

    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    
    let iter = IntoIter { iter: map.into_iter() };
    let _ = fmt::Debug::fmt(&iter, &mut fmt::Formatter::new());
}

#[test]
fn test_into_iter_debug_single_element() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = MockAllocator;
    let mut map: HashMap<i32, i32, _> = HashMap::with_hasher(DefaultHashBuilder::new());

    map.insert(1, 100);
    
    let iter = IntoIter { iter: map.into_iter() };
    let _ = fmt::Debug::fmt(&iter, &mut fmt::Formatter::new());
}

#[test]
fn test_into_iter_debug_multiple_elements() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = MockAllocator;
    let mut map: HashMap<&str, i32, _> = HashMap::with_hasher(DefaultHashBuilder::new());

    map.insert("a", 1);
    map.insert("b", 2);
    
    let iter = IntoIter { iter: map.into_iter() };
    let _ = fmt::Debug::fmt(&iter, &mut fmt::Formatter::new());
}

