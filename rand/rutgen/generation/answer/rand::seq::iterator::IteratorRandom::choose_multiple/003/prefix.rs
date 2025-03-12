// Answer 0

#[test]
fn test_choose_multiple_with_exact_amount() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Some(val)
            } else {
                None
            }
        }
    }

    let rng = DummyRng::new();
    let iterator = TestIterator {
        data: vec![1, 2, 3, 4, 5],
        index: 0,
    };
    let amount = 5;
    let result = iterator.choose_multiple(&mut rng, amount);
    // Result will be used in subsequent tests
}

#[test]
fn test_choose_multiple_with_less_than_amount() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Some(val)
            } else {
                None
            }
        }
    }

    let rng = DummyRng::new();
    let iterator = TestIterator {
        data: vec![1, 2, 3],
        index: 0,
    };
    let amount = 5;
    let result = iterator.choose_multiple(&mut rng, amount);
}

#[test]
fn test_choose_multiple_with_zero_amount() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Some(val)
            } else {
                None
            }
        }
    }

    let rng = DummyRng::new();
    let iterator = TestIterator {
        data: vec![1, 2, 3],
        index: 0,
    };
    let amount = 0;
    let result = iterator.choose_multiple(&mut rng, amount);
}

struct DummyRng {
    calls: usize,
}

impl DummyRng {
    fn new() -> Self {
        DummyRng { calls: 0 }
    }
}

impl Rng for DummyRng {
    fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
        self.calls += 1;
        range.start // Dummy implementation
    }
}

