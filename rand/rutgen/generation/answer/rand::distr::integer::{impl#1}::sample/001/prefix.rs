// Answer 0

#[test]
fn test_sample_with_valid_rng() {
    struct TestRng {
        state: u32,
    }

    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.state += 1;
            self.state
        }
    }

    let mut rng = TestRng { state: 0 };
    let uniform = StandardUniform;
    let value = uniform.sample(&mut rng);
}

#[test]
fn test_sample_with_high_rng_value() {
    struct HighValueRng {
        state: u32,
    }

    impl Rng for HighValueRng {
        fn next_u32(&mut self) -> u32 {
            self.state = u32::MAX;
            self.state
        }
    }

    let mut rng = HighValueRng { state: 0 };
    let uniform = StandardUniform;
    let value = uniform.sample(&mut rng);
}

#[test]
fn test_sample_with_boundary_case_zero() {
    struct ZeroRng;

    impl Rng for ZeroRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
    }

    let mut rng = ZeroRng;
    let uniform = StandardUniform;
    let value = uniform.sample(&mut rng);
}

#[test]
fn test_sample_with_boundary_case_max_u16() {
    struct MaxU16Rng {
        state: u32,
    }

    impl Rng for MaxU16Rng {
        fn next_u32(&mut self) -> u32 {
            65535 // value that is still within u16 limit when cast
        }
    }

    let mut rng = MaxU16Rng { state: 0 };
    let uniform = StandardUniform;
    let value = uniform.sample(&mut rng);
}

