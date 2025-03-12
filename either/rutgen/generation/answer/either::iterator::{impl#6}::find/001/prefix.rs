// Answer 0

#[test]
fn test_find_right_inner_some() {
    struct TestIter {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let result = self.items[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let iter = TestIter { items: vec![1, 2, 3, 4], index: 0 };
    let inner = Either::Right(iter);
    let mut iter_either = IterEither { inner };

    let result = iter_either.find(|&x| matches!(x, Either::Right(2)));
}

#[test]
fn test_find_right_inner_none() {
    struct TestIter {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let result = self.items[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let iter = TestIter { items: vec![1, 3, 4], index: 0 };
    let inner = Either::Right(iter);
    let mut iter_either = IterEither { inner };

    let result = iter_either.find(|&x| matches!(x, Either::Right(2)));
}

#[test]
fn test_find_right_inner_multiple_matches() {
    struct TestIter {
        items: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let result = self.items[self.index];
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let iter = TestIter { items: vec![2, 2, 3, 4], index: 0 };
    let inner = Either::Right(iter);
    let mut iter_either = IterEither { inner };

    let result = iter_either.find(|&x| matches!(x, Either::Right(2)));
}

