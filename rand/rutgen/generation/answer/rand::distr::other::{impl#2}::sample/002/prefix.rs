// Answer 0

#[test]
fn test_sample_var_equal_to_range() {
    struct TestRng {
        value: u32,
    }
    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }
    
    let rng = &mut TestRng { value: 62 };
    let alphanumeric = Alphanumeric;
    let result = alphanumeric.sample(rng);
}

#[test]
fn test_sample_var_less_than_range() {
    struct TestRng {
        value: u32,
    }
    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }
    
    let rng = &mut TestRng { value: 61 };
    let alphanumeric = Alphanumeric;
    let result = alphanumeric.sample(rng);
}

