// Answer 0

#[test]
#[should_panic]
fn test_random_ratio_numerator_greater_than_denominator() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 0;
            }
        }
        fn drop(&mut self) {}
    }

    let mut rng = TestRng;
    rng.random_ratio(3, 2);
}

#[test]
#[should_panic]
fn test_random_ratio_denominator_zero() {
    struct TestRng;

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 0;
            }
        }
        fn drop(&mut self) {}
    }

    let mut rng = TestRng;
    rng.random_ratio(1, 0);
}

