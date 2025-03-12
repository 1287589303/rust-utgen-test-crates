// Answer 0

#[test]
fn test_for_each_with_right_iterator() {
    struct RightIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIterator {
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

    let right_iter = RightIterator { items: vec![1, 2, 3], index: 0 };
    let either = Either::Right(right_iter);
    let mut sum = 0;

    either.for_each(|x| sum += x);
}

#[test]
fn test_for_each_with_empty_right_iterator() {
    struct EmptyRightIterator;

    impl Iterator for EmptyRightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let empty_right_iter = EmptyRightIterator;
    let either = Either::Right(empty_right_iter);
    let mut sum = 0;

    either.for_each(|x| sum += x);
}

#[test]
fn test_for_each_with_single_item_right_iterator() {
    struct SingleItemIterator {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for SingleItemIterator {
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

    let single_item_iter = SingleItemIterator { items: vec![42], index: 0 };
    let either = Either::Right(single_item_iter);
    let mut product = 1;

    either.for_each(|x| product *= x);
}

