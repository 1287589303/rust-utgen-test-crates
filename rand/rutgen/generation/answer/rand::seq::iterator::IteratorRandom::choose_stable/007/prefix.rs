// Answer 0

#[test]
fn test_choose_stable_with_lower_eq_2_and_no_valid_highest_selected() {
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
    }

    struct MockRng {
        value: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.value % (range.end - range.start) + range.start
        }
    }

    let mut rng = MockRng { value: 0 };
    let iterator = TestIterator {
        data: vec![1, 2],
        index: 0,
    };

    let result = iterator.choose_stable(&mut rng);
}

#[test]
fn test_choose_stable_with_valid_higest_selected_and_out_of_bounds() {
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
    }

    struct MockRng {
        value: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.value % (range.end - range.start) + range.start
        }
    }

    let mut rng = MockRng { value: 1 };
    let iterator = TestIterator {
        data: vec![1, 2],
        index: 0,
    };

    let result = iterator.choose_stable(&mut rng);
}

#[test]
fn test_choose_stable_no_item_in_nth() {
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

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.index += n;
            self.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.data.len() - self.index;
            (remaining, Some(remaining))
        }
    }

    struct MockRng {
        value: usize,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.value % (range.end - range.start) + range.start
        }
    }

    let mut rng = MockRng { value: 1 };
    let iterator = TestIterator {
        data: vec![1, 2],
        index: 0,
    };

    let result = iterator.choose_stable(&mut rng);
}

