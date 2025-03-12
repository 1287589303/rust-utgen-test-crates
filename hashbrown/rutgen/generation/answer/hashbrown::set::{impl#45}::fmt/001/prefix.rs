// Answer 0

#[test]
fn test_fmt_valid_intersection() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut hasher = DefaultHasher::new();
    1.hash(&mut hasher);
    let hash1 = hasher.finish();

    hasher = DefaultHasher::new();
    2.hash(&mut hasher);
    let hash2 = hasher.finish();

    let mut set1 = HashSet::<i32, DefaultHasher, SimpleAllocator>::with_capacity(2);
    set1.insert(1);
    set1.insert(2);

    let mut set2 = HashSet::<i32, DefaultHasher, SimpleAllocator>::with_capacity(2);
    set2.insert(2);
    set2.insert(3);

    let intersection = Intersection {
        iter: Iter {
            inner: RawIter::new(),
            marker: PhantomData,
        },
        other: &set2,
    };

    let formatter = &mut fmt::Formatter::default();
    let _ = intersection.fmt(formatter);
}

#[test]
fn test_fmt_empty_intersection() {
    use std::collections::hash_map::DefaultHasher;

    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let set1 = HashSet::<i32, DefaultHasher, SimpleAllocator>::new();
    let set2 = HashSet::<i32, DefaultHasher, SimpleAllocator>::new();

    let intersection = Intersection {
        iter: Iter {
            inner: RawIter::new(),
            marker: PhantomData,
        },
        other: &set2,
    };

    let formatter = &mut fmt::Formatter::default();
    let _ = intersection.fmt(formatter);
}

#[test]
fn test_fmt_single_element_intersection() {
    use std::collections::hash_map::DefaultHasher;

    struct SimpleAllocator;

    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut set1 = HashSet::<i32, DefaultHasher, SimpleAllocator>::with_capacity(1);
    set1.insert(1);

    let mut set2 = HashSet::<i32, DefaultHasher, SimpleAllocator>::with_capacity(1);
    set2.insert(1);
    set2.insert(2);

    let intersection = Intersection {
        iter: Iter {
            inner: RawIter::new(),
            marker: PhantomData,
        },
        other: &set2,
    };

    let formatter = &mut fmt::Formatter::default();
    let _ = intersection.fmt(formatter);
}

