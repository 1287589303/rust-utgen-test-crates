// Answer 0

#[test]
fn test_choose_case_1() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
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
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.data.len(), Some(self.data.len()))
        }
    }

    let mut rng = rand::rngs::OsRng;
    let iterator = TestIterator {
        data: vec![1, 2, 3, 4],
        index: 0,
    };
    let result = iterator.choose(&mut rng);
}

#[test]
fn test_choose_case_2() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
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

        fn size_hint(&self) -> (usize, Option<usize>) {
            (3, Some(10)) // lower > 1, upper > lower
        }
    }

    let mut rng = rand::rngs::OsRng;
    let iterator = TestIterator {
        data: vec![10, 20, 30],
        index: 0,
    };
    let result = iterator.choose(&mut rng);
}

#[test]
fn test_choose_case_3() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
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
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (1, Some(1)) // lower == 1
        }
    }

    let mut rng = rand::rngs::OsRng;
    let iterator = TestIterator {
        data: vec![5],
        index: 0,
    };
    let result = iterator.choose(&mut rng);
}

#[test]
fn test_choose_case_4() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None // empty iterator
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (1, Some(1)) // lower > 0, upper == lower
        }
    }

    let mut rng = rand::rngs::OsRng;
    let iterator = TestIterator {
        data: vec![],
        index: 0,
    };
    let result = iterator.choose(&mut rng);
}

