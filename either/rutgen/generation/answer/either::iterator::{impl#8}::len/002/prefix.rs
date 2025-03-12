// Answer 0

#[test]
fn test_len_left_empty() {
    struct EmptyIterator;
    impl ExactSizeIterator for EmptyIterator {
        fn len(&self) -> usize { 0 }
        fn is_empty(&self) -> bool { true }
    }

    let iter_empty = IterEither { inner: Either::Left(EmptyIterator) };
    let _result = iter_empty.len();
}

#[test]
fn test_len_left_one() {
    struct OneElementIterator {
        count: usize,
    }

    impl ExactSizeIterator for OneElementIterator {
        fn len(&self) -> usize { 1 }
        fn is_empty(&self) -> bool { false }
    }

    let iter_one = IterEither { inner: Either::Left(OneElementIterator { count: 1 }) };
    let _result = iter_one.len();
}

#[test]
fn test_len_left_two() {
    struct TwoElementsIterator {
        count: usize,
    }

    impl ExactSizeIterator for TwoElementsIterator {
        fn len(&self) -> usize { 2 }
        fn is_empty(&self) -> bool { false }
    }

    let iter_two = IterEither { inner: Either::Left(TwoElementsIterator { count: 2 }) };
    let _result = iter_two.len();
}

#[test]
fn test_len_left_max_size() {
    struct MaxElementsIterator {
        count: usize,
    }

    impl ExactSizeIterator for MaxElementsIterator {
        fn len(&self) -> usize { usize::MAX }
        fn is_empty(&self) -> bool { false }
    }

    let iter_max = IterEither { inner: Either::Left(MaxElementsIterator { count: usize::MAX }) };
    let _result = iter_max.len();
}

