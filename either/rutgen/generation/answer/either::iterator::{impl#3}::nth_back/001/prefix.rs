// Answer 0

#[test]
fn test_nth_back_non_negative() {
    struct RightIterator {
        data: Vec<i32>,
        index: usize,
    }
    impl Iterator for RightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    impl DoubleEndedIterator for RightIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.data[self.index])
            } else {
                None
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            if n == 0 {
                self.next_back()
            } else {
                None
            }
        }
    }

    let right_iter = RightIterator {
        data: vec![1, 2, 3],
        index: 3,
    };

    let either = Either::Right(right_iter);
    let result = either.nth_back(0);
}

#[test]
fn test_nth_back_out_of_bounds() {
    struct RightIterator {
        data: Vec<i32>,
        index: usize,
    }
    impl Iterator for RightIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    impl DoubleEndedIterator for RightIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.data[self.index])
            } else {
                None
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            if n >= self.data.len() {
                None
            } else {
                self.next_back()
            }
        }
    }

    let right_iter = RightIterator {
        data: vec![1, 2, 3],
        index: 3,
    };

    let either = Either::Right(right_iter);
    let result = either.nth_back(3);
}

