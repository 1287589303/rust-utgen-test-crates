// Answer 0

#[test]
fn test_rfind_with_non_empty_right_iterator() {
    struct TestIterator {
        current: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < 10 {
                let value = self.current;
                self.current += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    impl DoubleEndedIterator for TestIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.current > 0 {
                self.current -= 1;
                Some(self.current)
            } else {
                None
            }
        }
    }

    let mut right_inner = TestIterator { current: 0 };
    let mut iter = IterEither {
        inner: Either::Right(right_inner),
    };

    let found_item = iter.rfind(|&item| item % 2 == 0);
}

#[test]
fn test_rfind_with_empty_right_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl DoubleEndedIterator for EmptyIterator {}

    let mut right_inner = EmptyIterator;
    let mut iter = IterEither {
        inner: Either::Right(right_inner),
    };

    let found_item = iter.rfind(|&item| item % 2 == 0);
}

#[test]
fn test_rfind_with_right_iterator_single_element() {
    struct SingleElementIterator {
        value: usize,
        has_value: bool,
    }

    impl Iterator for SingleElementIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.has_value {
                self.has_value = false;
                Some(self.value)
            } else {
                None
            }
        }
    }

    impl DoubleEndedIterator for SingleElementIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.next()
        }
    }

    let mut right_inner = SingleElementIterator {
        value: 42,
        has_value: true,
    };
    
    let mut iter = IterEither {
        inner: Either::Right(right_inner),
    };

    let found_item = iter.rfind(|&item| item == 42);
}

