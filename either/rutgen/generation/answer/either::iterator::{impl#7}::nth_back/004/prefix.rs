// Answer 0

#[test]
fn test_nth_back_with_non_empty_left() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl DoubleEndedIterator for TestIterator {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.data[self.index])
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

    let left_iter = Either::Left(TestIterator {
        data: vec![1, 2, 3],
        index: 3,
    });
    let mut iter = IterEither { inner: left_iter };

    let result = iter.nth_back(1);
}

#[test]
fn test_nth_back_with_boundary_case_zero() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl DoubleEndedIterator for TestIterator {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.data[self.index])
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

    let left_iter = Either::Left(TestIterator {
        data: vec![5],
        index: 1,
    });
    let mut iter = IterEither { inner: left_iter };

    let result = iter.nth_back(0);
}

#[test]
fn test_nth_back_with_out_of_bounds() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl DoubleEndedIterator for TestIterator {
        type Item = i32;

        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.data[self.index])
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

    let left_iter = Either::Left(TestIterator {
        data: vec![10, 20, 30, 40],
        index: 4,
    });
    let mut iter = IterEither { inner: left_iter };

    let result = iter.nth_back(3);
}

