// Answer 0

#[test]
fn test_do_alloc_min_allocation() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let min_layout = Layout::from_size_align(1, 1).unwrap();
            if layout == min_layout {
                let ptr = unsafe { alloc(min_layout) };
                NonNull::new(ptr).ok_or(()).map(|p| p)
            } else {
                Err(())
            }
        }
    }
    
    let allocator = TestAllocator;
    let layout = Layout::from_size_align(1, 1).unwrap();
    let result = do_alloc(&allocator, layout);
}

#[test]
fn test_do_alloc_max_allocation() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let max_size = core::usize::MAX;
            let max_layout = Layout::from_size_align(max_size, max_size).unwrap();
            if layout == max_layout {
                let ptr = unsafe { alloc(max_layout) };
                NonNull::new(ptr).ok_or(()).map(|p| p)
            } else {
                Err(())
            }
        }
    }
    
    let allocator = TestAllocator;
    let layout = Layout::from_size_align(core::usize::MAX, core::usize::MAX).unwrap();
    let result = do_alloc(&allocator, layout);
}

#[test]
fn test_do_alloc_normal_allocation() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            let normal_size = 32;
            let normal_layout = Layout::from_size_align(normal_size, 8).unwrap();
            if layout == normal_layout {
                let ptr = unsafe { alloc(normal_layout) };
                NonNull::new(ptr).ok_or(()).map(|p| p)
            } else {
                Err(())
            }
        }
    }
    
    let allocator = TestAllocator;
    let layout = Layout::from_size_align(32, 8).unwrap();
    let result = do_alloc(&allocator, layout);
}

