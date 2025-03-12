// Answer 0

#[test]
fn test_with_state_ids_empty_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl ExactSizeIterator for EmptyIterator {
        fn len(&self) -> usize {
            0
        }
    }

    let _ = EmptyIterator.with_state_ids();
}

#[test]
fn test_with_state_ids_single_item_iterator() {
    struct SingleItemIterator {
        item: usize,
        done: bool,
    }

    impl Iterator for SingleItemIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.done {
                None
            } else {
                self.done = true;
                Some(self.item)
            }
        }
    }

    impl ExactSizeIterator for SingleItemIterator {
        fn len(&self) -> usize {
            1
        }
    }

    let _ = SingleItemIterator { item: 42, done: false }.with_state_ids();
}

#[test]
fn test_with_state_ids_multiple_items_iterator() {
    struct MultipleItemsIterator {
        count: usize,
    }

    impl Iterator for MultipleItemsIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for MultipleItemsIterator {
        fn len(&self) -> usize {
            self.count
        }
    }

    let _ = MultipleItemsIterator { count: 5 }.with_state_ids();
}

#[test]
fn test_with_state_ids_max_size_iterator() {
    struct MaxSizeIterator {
        count: usize,
        max: usize,
    }

    impl Iterator for MaxSizeIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    impl ExactSizeIterator for MaxSizeIterator {
        fn len(&self) -> usize {
            self.max
        }
    }

    let _ = MaxSizeIterator { count: 0, max: 100 }.with_state_ids();
}

