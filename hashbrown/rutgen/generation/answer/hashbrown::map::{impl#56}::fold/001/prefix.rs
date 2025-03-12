// Answer 0

#[test]
fn test_fold_with_integer_init_and_closure() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let allocator = TestAllocator;
    let data = vec![(1, 10), (2, 20), (3, 30)];
    let iter = IntoIter {
        inner: RawIntoIter {
            iter: RawIter::from_vec(data),
            allocation: None,
            marker: PhantomData,
        },
    };

    let result = iter.fold(0, |acc, (k, v)| acc + k + v);
}

#[test]
fn test_fold_with_string_init_and_closure() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let allocator = TestAllocator;
    let data = vec![(String::from("a"), String::from("alpha")),
                    (String::from("b"), String::from("beta"))];
    let iter = IntoIter {
        inner: RawIntoIter {
            iter: RawIter::from_vec(data),
            allocation: None,
            marker: PhantomData,
        },
    };

    let result = iter.fold(String::new(), |acc, (k, v)| acc + &k + &v);
}

#[test]
fn test_fold_with_empty_iterator() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    let allocator = TestAllocator;
    let data: Vec<(i32, i32)> = vec![];
    let iter = IntoIter {
        inner: RawIntoIter {
            iter: RawIter::from_vec(data),
            allocation: None,
            marker: PhantomData,
        },
    };

    let result = iter.fold(100, |acc, (k, v)| acc + k + v);
}

#[test]
fn test_fold_with_struct_init_and_closure() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            Ok(std::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    #[derive(Default)]
    struct Accumulator {
        total: i32,
    }

    let allocator = TestAllocator;
    let data = vec![(1, 5), (2, 10)];
    let iter = IntoIter {
        inner: RawIntoIter {
            iter: RawIter::from_vec(data),
            allocation: None,
            marker: PhantomData,
        },
    };

    let result = iter.fold(Accumulator::default(), |acc, (k, v)| {
        Accumulator {
            total: acc.total + k + v,
        }
    });
}

