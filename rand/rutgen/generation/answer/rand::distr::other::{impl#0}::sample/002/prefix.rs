// Answer 0

#[test]
fn test_sample_above_gap() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        // Implement necessary Rng methods here.
        fn random(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0xE000 }; // n is 0xE000 which is > 0xDFFF
    let distr: StandardUniform = StandardUniform;
    let result = distr.sample(&mut rng);
}

#[test]
fn test_sample_edge_case() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        // Implement necessary Rng methods here.
        fn random(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0x110000 }; // n is 0x110000 which is >= 0x110000
    let distr: StandardUniform = StandardUniform;
    let result = distr.sample(&mut rng);
}

#[test]
fn test_sample_inside_upper_bound() {
    struct MockRng {
        value: u32,
    }

    impl Rng for MockRng {
        // Implement necessary Rng methods here.
        fn random(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = MockRng { value: 0x10FFFF }; // n is 0x10FFFF, valid char value
    let distr: StandardUniform = StandardUniform;
    let result = distr.sample(&mut rng);
}

