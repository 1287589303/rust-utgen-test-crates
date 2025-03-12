// Answer 0

#[test]
fn test_unwrap_mut_success() {
    struct TestRng;

    impl TryRngCore for TestRng {
        type Error = ();

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(42)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(84)
        }

        fn try_fill_bytes(&mut self, _dst: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut rng = TestRng;
    let result = rng.unwrap_mut();
}

#[test]
fn test_unwrap_mut_multiple_times() {
    struct TestRng;

    impl TryRngCore for TestRng {
        type Error = ();

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(42)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(84)
        }

        fn try_fill_bytes(&mut self, _dst: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut rng = TestRng;
    let _result1 = rng.unwrap_mut();
    let _result2 = rng.unwrap_mut();
}

#[test]
fn test_unwrap_mut_empty_rng() {
    struct EmptyRng;

    impl TryRngCore for EmptyRng {
        type Error = ();

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(0)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(0)
        }

        fn try_fill_bytes(&mut self, _dst: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut rng = EmptyRng;
    let result = rng.unwrap_mut();
}

