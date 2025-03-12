// Answer 0

#[test]
fn test_sample_minimum_values() {
    struct MockRng {
        call_count: usize,
    }

    impl Rng for MockRng {
        fn next_u64(&mut self) -> u64 {
            self.call_count += 1;
            if self.call_count == 1 {
                0 // First call returns minimum value
            } else {
                0 // Second call returns minimum value
            }
        }
    }

    let rng = &mut MockRng { call_count: 0 };
    let uniform = StandardUniform;
    uniform.sample(rng);
}

#[test]
fn test_sample_large_values() {
    struct MockRng {
        call_count: usize,
    }

    impl Rng for MockRng {
        fn next_u64(&mut self) -> u64 {
            self.call_count += 1;
            if self.call_count == 1 {
                u64::MAX // First call returns maximum value
            } else {
                u64::MAX // Second call returns maximum value
            }
        }
    }

    let rng = &mut MockRng { call_count: 0 };
    let uniform = StandardUniform;
    uniform.sample(rng);
}

#[test]
fn test_sample_mixed_values() {
    struct MockRng {
        call_count: usize,
    }

    impl Rng for MockRng {
        fn next_u64(&mut self) -> u64 {
            self.call_count += 1;
            match self.call_count {
                1 => 42,    // Arbitrary non-boundary value
                2 => 18446744073709551615, // max u64
                _ => unreachable!(),
            }
        }
    }

    let rng = &mut MockRng { call_count: 0 };
    let uniform = StandardUniform;
    uniform.sample(rng);
}

#[test]
fn test_sample_zero_and_max() {
    struct MockRng {
        call_count: usize,
    }

    impl Rng for MockRng {
        fn next_u64(&mut self) -> u64 {
            self.call_count += 1;
            if self.call_count == 1 {
                0 // First call returns minimum value
            } else {
                u64::MAX // Second call returns maximum value
            }
        }
    }

    let rng = &mut MockRng { call_count: 0 };
    let uniform = StandardUniform;
    uniform.sample(rng);
}

