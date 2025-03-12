// Answer 0

#[test]
fn test_drain_non_empty_integers() {
    let mut set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let drain_iter = set.drain();
}

#[test]
fn test_drain_non_empty_strings() {
    let mut set: HashSet<&str> = ["one", "two", "three"].iter().cloned().collect();
    let drain_iter = set.drain();
}

#[test]
fn test_drain_with_different_allocator() {
    use hashbrown::raw::Global;

    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut set: HashSet<i32, DefaultHashBuilder, CustomAllocator> = [1, 2, 3].into_iter().collect();
    let drain_iter = set.drain();
}

#[test]
fn test_drain_empty_set() {
    let mut set: HashSet<i32> = HashSet::new();
    let drain_iter = set.drain();
}

#[test]
fn test_drain_large_set() {
    let mut set: HashSet<i32> = (0..100_000).collect();
    let drain_iter = set.drain();
}

