// Answer 0

#[test]
fn test_size_hint_with_right_iterator_empty() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right_iter = EmptyIterator;
    let iter_either = IterEither {
        inner: Either::Right(right_iter),
    };

    let _ = iter_either.size_hint();
}

#[test]
fn test_size_hint_with_right_iterator_single_element() {
    struct SingleElementIterator {
        count: usize,
    }

    impl Iterator for SingleElementIterator {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1)
            } else {
                None
            }
        }
    }

    let right_iter = SingleElementIterator { count: 1 };
    let iter_either = IterEither {
        inner: Either::Right(right_iter),
    };

    let _ = iter_either.size_hint();
}

#[test]
fn test_size_hint_with_right_iterator_multiple_elements() {
    struct MultipleElementsIterator {
        count: usize,
    }

    impl Iterator for MultipleElementsIterator {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1)
            } else {
                None
            }
        }
    }

    let right_iter = MultipleElementsIterator { count: 5 };
    let iter_either = IterEither {
        inner: Either::Right(right_iter),
    };

    let _ = iter_either.size_hint();
}

#[test]
fn test_size_hint_with_left_iterator() {
    struct SingleElementLeftIterator {
        count: usize,
    }

    impl Iterator for SingleElementLeftIterator {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1)
            } else {
                None
            }
        }
    }

    let left_iter = SingleElementLeftIterator { count: 1 };
    let iter_either = IterEither {
        inner: Either::Left(left_iter),
    };

    let _ = iter_either.size_hint();
}

