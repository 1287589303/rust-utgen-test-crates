// Answer 0

#[test]
fn test_iterator_len_hint_none_due_to_size_hint_second_none() {
    struct TestIterator {
        count: usize,
        current: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.count {
                let item = self.current;
                self.current += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.current, None)
        }
    }

    let iter = TestIterator { count: 10, current: 0 };
    let result = iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_none_due_to_size_hint_first_less_than_second() {
    struct TestIterator {
        count: usize,
        current: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.count {
                let item = self.current;
                self.current += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (2, Some(5))
        }
    }

    let iter = TestIterator { count: 10, current: 0 };
    let result = iterator_len_hint(&iter);
}

#[test]
fn test_iterator_len_hint_none_due_to_empty_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(5))
        }
    }

    let iter = EmptyIterator;
    let result = iterator_len_hint(&iter);
}

