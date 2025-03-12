// Answer 0

#[test]
fn test_next_back_empty_right_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl DoubleEndedIterator for EmptyIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let inner = Either::Right(EmptyIterator);
    let mut iter = IterEither { inner };

    let result = iter.next_back();
}

#[test]
fn test_next_back_err_right_iterator() {
    struct ErrIterator;

    impl Iterator for ErrIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl DoubleEndedIterator for ErrIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            Err(())
        }
    }

    let inner = Either::Right(ErrIterator);
    let mut iter = IterEither { inner };

    let result = iter.next_back();
}

