// Answer 0

#[test]
fn test_for_each_right_iterator() {
    struct RightIterator {
        count: usize,
    }

    impl Iterator for RightIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                let value = self.count;
                self.count += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let right_iterator = RightIterator { count: 0 };
    let iter_either = IterEither {
        inner: Either::Right(right_iterator),
    };

    iter_either.for_each(|item| {
        let _ = item; // Reference to item to avoid unused variable warning
    });
}

#[test]
fn test_for_each_right_iterator_empty() {
    struct RightIteratorEmpty;

    impl Iterator for RightIteratorEmpty {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right_iterator_empty = RightIteratorEmpty;
    let iter_either_empty = IterEither {
        inner: Either::Right(right_iterator_empty),
    };

    iter_either_empty.for_each(|item| {
        let _ = item; // Reference to item to avoid unused variable warning
    });
}

