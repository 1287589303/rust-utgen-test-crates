// Answer 0

#[test]
fn test_any_with_empty_right_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let right = Either::Right(EmptyIterator {});
    let result = right.any(|x| x > 5);
}

#[test]
fn test_any_with_single_item_right_iterator() {
    struct SingleIterator {
        item: i32,
        called: bool,
    }

    impl Iterator for SingleIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.called {
                None
            } else {
                self.called = true;
                Some(self.item)
            }
        }
    }

    let right = Either::Right(SingleIterator { item: 10, called: false });
    let result = right.any(|x| x > 5);
}

#[test]
fn test_any_with_multiple_items_right_iterator() {
    struct MultipleIterator {
        count: usize,
    }

    impl Iterator for MultipleIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 5 {
                None
            } else {
                self.count += 1;
                Some(self.count)
            }
        }
    }

    let right = Either::Right(MultipleIterator { count: 0 });
    let result = right.any(|x| x == 3);
}

#[test]
fn test_any_with_large_right_iterator() {
    struct LargeIterator {
        count: usize,
    }

    impl Iterator for LargeIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count >= 1000 {
                None
            } else {
                self.count += 1;
                Some(self.count)
            }
        }
    }

    let right = Either::Right(LargeIterator { count: 0 });
    let result = right.any(|x| x == 999);
}

