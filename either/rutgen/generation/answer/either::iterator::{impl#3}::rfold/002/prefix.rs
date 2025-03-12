// Answer 0

#[test]
fn test_rfold_with_left_only() {
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

    impl DoubleEndedIterator for LeftIter {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.values[self.index])
            } else {
                None
            }
        }

        fn rfold<Acc, G>(self, init: Acc, f: G) -> Acc
        where
            G: FnMut(Acc, Self::Item) -> Acc {
            let mut acc = init;
            for value in self.values.iter().rev() {
                acc = f(acc, *value);
            }
            acc
        }
    }

    let left_iter = LeftIter { values: vec![1, 2, 3], index: 0 };
    let either = Either::Left(left_iter);

    let result = either.rfold(0, |acc, x| acc + x);
}

#[test]
fn test_rfold_with_right_only() {
    struct RightIter {
        values: Vec<f32>,
        index: usize,
    }

    impl Iterator for RightIter {
        type Item = f32;

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

    impl DoubleEndedIterator for RightIter {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.values[self.index])
            } else {
                None
            }
        }

        fn rfold<Acc, G>(self, init: Acc, f: G) -> Acc
        where
            G: FnMut(Acc, Self::Item) -> Acc {
            let mut acc = init;
            for value in self.values.iter().rev() {
                acc = f(acc, *value);
            }
            acc
        }
    }

    let right_iter = RightIter { values: vec![1.0, 2.0, 3.0], index: 0 };
    let either = Either::Right(right_iter);

    let result = either.rfold(0.0, |acc, x| acc + x);
}

#[test]
fn test_rfold_with_both_left_and_right() {
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

    impl DoubleEndedIterator for LeftIter {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.values[self.index])
            } else {
                None
            }
        }
    }
    
    struct RightIter {
        values: Vec<f32>,
        index: usize,
    }

    impl Iterator for RightIter {
        type Item = f32;

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

    impl DoubleEndedIterator for RightIter {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index > 0 {
                self.index -= 1;
                Some(self.values[self.index])
            } else {
                None
            }
        }
    }

    let left_iter = LeftIter { values: vec![1, 2], index: 0 };
    let right_iter = RightIter { values: vec![3.0, 4.0], index: 0 };
    let either = Either::Left(left_iter);

    let result = either.rfold(0, |acc, x| acc + x);
    
    let either = Either::Right(right_iter);
    
    let result = either.rfold(0.0, |acc, x| acc + x);
}

