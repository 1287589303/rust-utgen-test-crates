// Answer 0

#[test]
fn test_fill_bytes_via_next_short_dest() {
    struct TestRng {
        value: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.copy_from_slice(&self.next_u32().to_le_bytes());
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = TestRng { value: 0x123456789abcdef0 };
    let mut dest: [u8; 4] = [0; 4];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_exact_four_bytes() {
    struct TestRng {
        value: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.copy_from_slice(&self.next_u32().to_le_bytes());
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = TestRng { value: 0xdeadbeef };
    let mut dest: [u8; 4] = [0; 4];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_empty() {
    struct TestRng {
        value: u64,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.copy_from_slice(&self.next_u32().to_le_bytes());
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = TestRng { value: 0x1 };
    let mut dest: [u8; 0] = [];
    fill_bytes_via_next(&mut rng, &mut dest);
}

