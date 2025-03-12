// Answer 0

#[test]
fn test_try_from_rng_success() {
    struct TestRng {
        bytes: Vec<u8>,
        filled: bool,
    }

    impl TryRngCore for TestRng {
        type Error = ();

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
            if self.filled {
                let to_fill = std::cmp::min(dest.len(), self.bytes.len());
                dest[..to_fill].copy_from_slice(&self.bytes[..to_fill]);
                Ok(())
            } else {
                Err(())
            }
        }
    }

    impl SeedableRng for TestRng {
        type Seed = [u8; 16];

        fn from_seed(seed: Self::Seed) -> Self {
            // Normally you'd construct a RNG from the seed
            TestRng { bytes: Vec::new(), filled: false }
        }
    }

    let mut rng = TestRng {
        bytes: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        filled: true,
    };
    let _ = TestRng::try_from_rng(&mut rng);
}

#[test]
fn test_try_from_rng_failure() {
    struct TestRng {
        bytes: Vec<u8>,
        filled: bool,
    }

    impl TryRngCore for TestRng {
        type Error = ();

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
            if self.filled {
                let to_fill = std::cmp::min(dest.len(), self.bytes.len());
                dest[..to_fill].copy_from_slice(&self.bytes[..to_fill]);
                Ok(())
            } else {
                Err(())
            }
        }
    }

    impl SeedableRng for TestRng {
        type Seed = [u8; 16];

        fn from_seed(seed: Self::Seed) -> Self {
            TestRng { bytes: Vec::new(), filled: false }
        }
    }

    let mut rng = TestRng {
        bytes: vec![],
        filled: false,
    };
    let _ = TestRng::try_from_rng(&mut rng).unwrap_err();
}

#[test]
fn test_try_from_rng_boundary_case() {
    struct TestRng {
        bytes: Vec<u8>,
        filled: bool,
    }

    impl TryRngCore for TestRng {
        type Error = ();

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
            if self.filled {
                let to_fill = std::cmp::min(dest.len(), 16); // Boundary case for size
                dest[..to_fill].copy_from_slice(&self.bytes[..to_fill]);
                Ok(())
            } else {
                Err(())
            }
        }
    }

    impl SeedableRng for TestRng {
        type Seed = [u8; 16];

        fn from_seed(seed: Self::Seed) -> Self {
            TestRng { bytes: Vec::new(), filled: false }
        }
    }

    let mut rng = TestRng {
        bytes: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        filled: true,
    };
    let _ = TestRng::try_from_rng(&mut rng);
}

