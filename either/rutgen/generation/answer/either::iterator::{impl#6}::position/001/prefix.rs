// Answer 0

#[test]
fn test_position_empty_iterator() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
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

    let inner = Either::Right(TestIter { data: vec![], index: 0 });
    let mut iter = IterEither { inner };

    let result = iter.position(|_| true);
}

#[test]
fn test_position_single_element_matching() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
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

    let inner = Either::Right(TestIter { data: vec![42], index: 0 });
    let mut iter = IterEither { inner };

    let result = iter.position(|item| item == Either::Right(42));
}

#[test]
fn test_position_single_element_non_matching() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
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

    let inner = Either::Right(TestIter { data: vec![42], index: 0 });
    let mut iter = IterEither { inner };

    let result = iter.position(|item| item == Either::Right(100));
}

#[test]
fn test_position_multi_element_first_match() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
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

    let inner = Either::Right(TestIter { data: vec![1, 2, 3], index: 0 });
    let mut iter = IterEither { inner };

    let result = iter.position(|item| item == Either::Right(1));
}

#[test]
fn test_position_multi_element_last_match() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
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

    let inner = Either::Right(TestIter { data: vec![1, 2, 3], index: 0 });
    let mut iter = IterEither { inner };

    let result = iter.position(|item| item == Either::Right(3));
}

#[test]
fn test_position_multi_element_mid_match() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
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

    let inner = Either::Right(TestIter { data: vec![1, 2, 3], index: 0 });
    let mut iter = IterEither { inner };

    let result = iter.position(|item| item == Either::Right(2));
}

