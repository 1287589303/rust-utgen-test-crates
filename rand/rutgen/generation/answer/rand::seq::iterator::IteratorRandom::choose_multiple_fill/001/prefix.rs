// Answer 0

#[test]
fn test_choose_multiple_fill_some_elements() {
    struct TestRng {
        value: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.value % range.end // Using modulo for simplicity
        }
    }

    struct TestIterator {
        current: usize,
        max: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let val = self.current;
                self.current += 1;
                Some(val)
            } else {
                None
            }
        }
    }

    let mut rng = TestRng { value: 5 };
    let mut buf = vec![0; 10]; // Buffer of size 10
    let iterator = TestIterator { current: 0, max: 5 }; // Iterator with 5 elements

    let len = iterator.choose_multiple_fill(&mut rng, &mut buf);

    // The length should be 5, as we have fewer elements than the buffer size
    assert_eq!(len, 5);
}

#[test]
fn test_choose_multiple_fill_exhausted_iterator() {
    struct TestRng {
        value: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.value % range.end // Use modulo for simplification
        }
    }

    struct TestIterator {
        current: usize,
        max: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let val = self.current;
                self.current += 1;
                Some(val)
            } else {
                None
            }
        }
    }

    let mut rng = TestRng { value: 5 };
    let mut buf = vec![0; 5]; // Buffer of size 5
    let iterator = TestIterator { current: 0, max: 5 }; // Iterator with 5 elements

    let len = iterator.choose_multiple_fill(&mut rng, &mut buf);

    // The length should be 5, as the iterator has precisely enough elements
    assert_eq!(len, 5);
}

#[test]
fn test_choose_multiple_fill_partially_filled_buffer() {
    struct TestRng {
        value: usize,
    }

    impl Rng for TestRng {
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            self.value % range.end // Simple modulo for range
        }
    }

    struct TestIterator {
        current: usize,
        max: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let val = self.current;
                self.current += 1;
                Some(val)
            } else {
                None
            }
        }
    }

    let mut rng = TestRng { value: 5 };
    let mut buf = vec![0; 10]; // Buffer of size 10
    let iterator = TestIterator { current: 0, max: 3 }; // Iterator with only 3 elements

    let len = iterator.choose_multiple_fill(&mut rng, &mut buf);

    // The length should be 3, as the iterator does not have enough elements to fill the buffer
    assert_eq!(len, 3);
}

