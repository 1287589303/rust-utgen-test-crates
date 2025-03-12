// Answer 0

#[test]
fn test_random_iter_integers() {
    struct MockRng {
        state: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.state += 1;
            self.state
        }

        fn next_u64(&mut self) -> u64 {
            (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = (self.next_u32() % 256) as u8;
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let rng = MockRng { state: 0 };
    let v: Vec<u32> = rng.random_iter().take(10).collect();
}

#[test]
fn test_random_iter_floats() {
    struct MockRng {
        state: f32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.state += 0.1;
            (self.state * 100.0) as u32
        }

        fn next_u64(&mut self) -> u64 {
            (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = (self.next_u32() % 256) as u8;
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let rng = MockRng { state: 0.0 };
    let v: Vec<f32> = rng.random_iter().take(10).collect();
}

#[test]
fn test_random_iter_bounding() {
    struct MockRng {
        state: u32,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.state += 1;
            self.state
        }

        fn next_u64(&mut self) -> u64 {
            (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = (self.next_u32() % 256) as u8;
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let rng = MockRng { state: u32::MAX - 10 };
    let v: Vec<u32> = rng.random_iter().take(10).collect();
}

