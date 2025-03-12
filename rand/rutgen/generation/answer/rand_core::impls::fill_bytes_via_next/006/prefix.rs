// Answer 0

#[test]
fn test_fill_bytes_via_next_length_zero() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { dest.fill(0); }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = TestRng;
    let mut dest: [u8; 0] = [];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_length_four() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 1 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { dest.fill(0); }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = TestRng;
    let mut dest: [u8; 4] = [0; 4];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_length_three() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 1 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { dest.fill(0); }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = TestRng;
    let mut dest: [u8; 3] = [0; 3];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_length_two() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 1 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { dest.fill(0); }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = TestRng;
    let mut dest: [u8; 2] = [0; 2];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_length_one() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 { 1 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) { dest.fill(0); }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = TestRng;
    let mut dest: [u8; 1] = [0; 1];
    fill_bytes_via_next(&mut rng, &mut dest);
}

