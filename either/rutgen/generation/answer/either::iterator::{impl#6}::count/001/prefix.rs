// Answer 0

#[test]
fn test_count_with_empty_right_iterator() {
    struct EmptyIter;

    impl Iterator for EmptyIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let iter_either = IterEither {
        inner: Either::Right(EmptyIter),
    };

    let result = iter_either.count();
}

#[test]
fn test_count_with_single_item_right_iterator() {
    struct SingleItemIter {
        item: Option<i32>,
    }

    impl Iterator for SingleItemIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.item.take()
        }
    }

    let iter_either = IterEither {
        inner: Either::Right(SingleItemIter { item: Some(1) }),
    };

    let result = iter_either.count();
}

#[test]
fn test_count_with_multiple_items_right_iterator() {
    struct MultipleItemsIter {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for MultipleItemsIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let iter_either = IterEither {
        inner: Either::Right(MultipleItemsIter {
            items: vec![1, 2, 3],
            index: 0,
        }),
    };

    let result = iter_either.count();
}

#[test]
fn test_count_with_unbounded_right_iterator() {
    struct UnboundedIter {
        count: usize,
    }

    impl Iterator for UnboundedIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            Some(self.count)
        }
    }

    let iter_either = IterEither {
        inner: Either::Right(UnboundedIter { count: 0 }),
    };

    let result = iter_either.count();
}

