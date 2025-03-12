// Answer 0

#[test]
fn test_len_zero_elements() {
    struct TestIterator {
        count: usize,
    }

    impl ExactSizeIterator for TestIterator {
        fn len(&self) -> usize {
            self.count
        }
    }

    let inner = Either::Right(TestIterator { count: 0 });
    let iter_either = IterEither { inner };
    let _ = iter_either.len();
}

#[test]
fn test_len_five_elements() {
    struct TestIterator {
        count: usize,
    }

    impl ExactSizeIterator for TestIterator {
        fn len(&self) -> usize {
            self.count
        }
    }

    let inner = Either::Right(TestIterator { count: 5 });
    let iter_either = IterEither { inner };
    let _ = iter_either.len();
}

#[test]
fn test_len_maximum_elements() {
    struct TestIterator {
        count: usize,
    }

    impl ExactSizeIterator for TestIterator {
        fn len(&self) -> usize {
            self.count
        }
    }

    let inner = Either::Right(TestIterator { count: usize::MAX });
    let iter_either = IterEither { inner };
    let _ = iter_either.len();
}

