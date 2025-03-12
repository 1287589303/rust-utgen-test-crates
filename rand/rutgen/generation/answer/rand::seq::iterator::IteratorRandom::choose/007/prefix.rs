// Answer 0

#[test]
fn test_choose_with_upper_equal_lower() {
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
            let len = self.data.len() - self.index;
            (len, Some(len))
        }
    }

    let mut rng = rand::thread_rng();
    let iterator = TestIterator {
        data: vec![1, 2, 3],
        index: 0,
    };

    let result = iterator.choose(&mut rng);
}

#[test]
fn test_choose_with_lower_greater_than_one() {
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
            let len = self.data.len() - self.index;
            (len, Some(len))
        }
    }

    let mut rng = rand::thread_rng();
    let iterator = TestIterator {
        data: vec![1, 2, 3, 4],
        index: 0,
    };

    let result = iterator.choose(&mut rng);
}

#[test]
fn test_choose_with_ix_equal_lower_and_upper() {
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
            let len = self.data.len() - self.index;
            (len, Some(len))
        }
    }

    let mut rng = rand::thread_rng();
    let iterator = TestIterator {
        data: vec![1, 2],
        index: 0,
    };

    let result = iterator.choose(&mut rng);
}

