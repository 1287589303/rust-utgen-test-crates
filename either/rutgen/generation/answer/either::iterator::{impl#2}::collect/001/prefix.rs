// Answer 0

#[test]
fn test_collect_right_simple() {
    struct TestRight {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestRight {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let right_iter = TestRight { data: vec![1, 2, 3], index: 0 };
    let either = Either::Right(right_iter);
    let collected: Vec<i32> = either.collect();
}

#[test]
fn test_collect_right_empty() {
    struct TestRight {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestRight {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let right_iter = TestRight { data: vec![], index: 0 };
    let either = Either::Right(right_iter);
    let collected: Vec<i32> = either.collect();
}

#[test]
fn test_collect_right_different_type() {
    struct TestRight {
        data: Vec<u32>,
        index: usize,
    }

    impl Iterator for TestRight {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let right_iter = TestRight { data: vec![1, 2], index: 0 };
    let either = Either::Right(right_iter);
    let collected: Vec<u32> = either.collect();
}

#[test]
fn test_collect_right_large() {
    struct TestRight {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestRight {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let result = self.data[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let right_iter = TestRight { data: (1..1000).collect(), index: 0 };
    let either = Either::Right(right_iter);
    let collected: Vec<i32> = either.collect();
}

