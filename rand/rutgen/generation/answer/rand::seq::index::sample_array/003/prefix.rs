// Answer 0

#[test]
fn test_sample_array_n_equals_len() {
    struct MockRng {
        counter: usize,
    }

    impl Rng for MockRng {
        #[inline]
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let value = if self.counter < range.end {
                self.counter
            } else {
                range.start // Loop back to start to avoid out of range
            };
            self.counter += 1;
            value
        }
    }

    let len = 5;
    let n = 5;
    let mut rng = MockRng { counter: 0 };

    let result = sample_array::<MockRng, 5>(&mut rng, len);
    
    // Call the function without asserting
    let _ = result;
}

#[test]
fn test_sample_array_n_less_than_len() {
    struct MockRng {
        counter: usize,
    }

    impl Rng for MockRng {
        #[inline]
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let value = if self.counter < range.end {
                self.counter
            } else {
                range.start // Loop back to start to avoid out of range
            };
            self.counter += 1;
            value
        }
    }

    let len = 6;
    let n = 5;
    let mut rng = MockRng { counter: 0 };

    let result = sample_array::<MockRng, 5>(&mut rng, len);

    // Call the function without asserting
    let _ = result;
}

#[test]
fn test_sample_array_with_random_values() {
    struct MockRng {
        random_values: Vec<usize>,
        index: usize,
    }

    impl Rng for MockRng {
        #[inline]
        fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
            let value = self.random_values[self.index % self.random_values.len()];
            self.index += 1;
            value % range.end
        }
    }

    let len = 10;
    let n = 5;
    let mut rng = MockRng {
        random_values: vec![3, 1, 4, 2, 0, 5, 9, 8, 7, 6],
        index: 0,
    };

    let result = sample_array::<MockRng, 5>(&mut rng, len);

    // Call the function without asserting
    let _ = result;
}

