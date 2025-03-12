// Answer 0

#[test]
fn test_sample_alphabetic_uppercase() {
    struct MockRng {
        value: u8,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<u8>) -> u8 {
            self.value = range.start; // set to minimum for testing
            self.value
        }
    }

    let mut rng = MockRng { value: 0 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
}

#[test]
fn test_sample_alphabetic_lowercase() {
    struct MockRng {
        value: u8,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<u8>) -> u8 {
            self.value = range.end - 1; // set to maximum for testing
            self.value
        }
    }

    let mut rng = MockRng { value: 0 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
}

#[test]
fn test_sample_alphabetic_boundary() {
    struct MockRng {
        value: u8,
    }

    impl Rng for MockRng {
        fn random_range(&mut self, range: std::ops::Range<u8>) -> u8 {
            self.value = range.start + 26; // test middle of the range
            self.value
        }
    }

    let mut rng = MockRng { value: 0 };
    let alphabetic = Alphabetic;
    let result = alphabetic.sample(&mut rng);
}

