// Answer 0

#[test]
fn test_any_with_left_iterator() {
    struct LeftIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    struct RightIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { values: vec![1, 2, 3], index: 0 };
    let right_iter = RightIter { values: vec![4, 5], index: 0 };
    let either = Either::Left(left_iter);

    let result = either.any(|x| x > 2);
}

#[test]
fn test_any_with_right_iterator() {
    struct LeftIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for LeftIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    struct RightIter {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for RightIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { values: vec![5, 6], index: 0 };
    let right_iter = RightIter { values: vec![1, 2, 3], index: 0 };
    let either = Either::Right(right_iter);

    let result = either.any(|x| x < 2);
}

