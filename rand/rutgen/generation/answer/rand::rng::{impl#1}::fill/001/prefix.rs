// Answer 0

#[test]
fn test_fill_with_non_empty_array() {
    struct MockRng;
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 { 42 }
        fn next_u64(&mut self) -> u64 { 42 }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 42;
            }
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut array: [u8; 10] = [0; 10];
    let mut rng = MockRng;
    array.fill(&mut rng);
}

#[test]
fn test_fill_with_non_empty_slice() {
    struct MockRng;
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 { 42 }
        fn next_u64(&mut self) -> u64 { 42 }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 42;
            }
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut slice: &mut [u8] = &mut [0; 5];
    let mut rng = MockRng;
    slice.fill(&mut rng);
}

#[test]
#[should_panic]
fn test_fill_with_zero_length_array() {
    struct MockRng;
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 { 42 }
        fn next_u64(&mut self) -> u64 { 42 }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 42;
            }
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut array: [u8; 0] = [];
    let mut rng = MockRng;
    array.fill(&mut rng);
}

