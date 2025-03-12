// Answer 0

#[test]
fn test_nth_back_left_valid_index() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl DoubleEndedIterator for TestIterator {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                self.data.get(self.index).copied()
            } else {
                None
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            if n < self.data.len() {
                Some(self.data[self.data.len() - 1 - n])
            } else {
                None
            }
        }
    }

    let left_iter = TestIterator {
        data: vec![1, 2, 3, 4, 5],
        index: 5,
    };

    let mut either = Either::Left(left_iter);
    let result = either.nth_back(1);
}

#[test]
fn test_nth_back_left_boundary_case_zero() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl DoubleEndedIterator for TestIterator {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                self.data.get(self.index).copied()
            } else {
                None
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            if n < self.data.len() {
                Some(self.data[self.data.len() - 1 - n])
            } else {
                None
            }
        }
    }

    let left_iter = TestIterator {
        data: vec![1, 2, 3, 4, 5],
        index: 5,
    };

    let mut either = Either::Left(left_iter);
    let result = either.nth_back(0);
}

#[test]
fn test_nth_back_left_boundary_case_last() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl DoubleEndedIterator for TestIterator {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                self.data.get(self.index).copied()
            } else {
                None
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            if n < self.data.len() {
                Some(self.data[self.data.len() - 1 - n])
            } else {
                None
            }
        }
    }

    let left_iter = TestIterator {
        data: vec![1, 2, 3, 4, 5],
        index: 5,
    };

    let mut either = Either::Left(left_iter);
    let result = either.nth_back(4);
}

