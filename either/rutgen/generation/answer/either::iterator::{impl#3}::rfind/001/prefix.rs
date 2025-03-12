// Answer 0

#[test]
fn test_rfind_with_all_matching_elements() {
    struct LeftIter {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
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

    struct RightIter {
        items: Vec<i32>,
        index: usize,
    }

    impl DoubleEndedIterator for RightIter {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.items[self.index])
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { items: vec![1, 2, 3], index: 0 };
    let right_iter = RightIter { items: vec![1, 2, 3], index: 3 };

    let mut either = Either::Right(right_iter);

    let result = either.rfind(|&x| x == 2);
}

#[test]
fn test_rfind_with_none_matching_elements() {
    struct LeftIter {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
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

    struct RightIter {
        items: Vec<i32>,
        index: usize,
    }

    impl DoubleEndedIterator for RightIter {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.items[self.index])
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { items: vec![4, 5, 6], index: 0 };
    let right_iter = RightIter { items: vec![4, 5, 6], index: 3 };

    let mut either = Either::Right(right_iter);

    let result = either.rfind(|&x| x == 7);
}

#[test]
fn test_rfind_with_partial_matching_elements() {
    struct LeftIter {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
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

    struct RightIter {
        items: Vec<i32>,
        index: usize,
    }

    impl DoubleEndedIterator for RightIter {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.items[self.index])
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { items: vec![1, 2, 3], index: 0 };
    let right_iter = RightIter { items: vec![1, 2, 4], index: 3 };

    let mut either = Either::Right(right_iter);

    let result = either.rfind(|&x| x == 2);
}

