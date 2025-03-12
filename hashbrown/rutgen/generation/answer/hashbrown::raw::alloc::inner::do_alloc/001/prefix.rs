// Answer 0

#[derive(Clone)]
struct MockAllocator;

impl Allocator for MockAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
        Err(())
    }
}

#[test]
fn test_do_alloc_with_invalid_layout() {
    let allocator = MockAllocator;
    let layout = Layout::from_size_align(1, 1).unwrap(); // Normally valid, just for base. 

    let result = do_alloc(&allocator, layout);
}

#[test]
fn test_do_alloc_with_large_layout() {
    let allocator = MockAllocator;
    let layout = Layout::from_size_align(usize::MAX, 1).unwrap(); // Unreasonably large for allocation.

    let result = do_alloc(&allocator, layout);
}

#[test]
fn test_do_alloc_with_zero_size() {
    let allocator = MockAllocator;
    let layout = Layout::from_size_align(0, 1).unwrap(); // A zero-sized request which could be invalid.

    let result = do_alloc(&allocator, layout);
}

#[test]
fn test_do_alloc_with_unaligned_layout() {
    let allocator = MockAllocator;
    let layout = Layout::from_size_align(4, 3).unwrap(); // Misaligned request (alignment must be a divisor of size).

    let result = do_alloc(&allocator, layout);
}

