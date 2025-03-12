// Answer 0

#[test]
fn test_fill_bytes_min_size() {
    struct MockRng;
    
    impl TryRngCore for MockRng {
        type Error = ();
        
        fn try_next_u32(&mut self) -> Result<u32, Self::Error> { Ok(0) }
        fn try_next_u64(&mut self) -> Result<u64, Self::Error> { Ok(0) }
        
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            dst.fill(1);
            Ok(())
        }
    }

    let mut rng = UnwrapErr(MockRng);
    let mut dst = [0u8; 1];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_max_size() {
    struct MockRng;
    
    impl TryRngCore for MockRng {
        type Error = ();
        
        fn try_next_u32(&mut self) -> Result<u32, Self::Error> { Ok(0) }
        fn try_next_u64(&mut self) -> Result<u64, Self::Error> { Ok(0) }
        
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            dst.fill(2);
            Ok(())
        }
    }

    let mut rng = UnwrapErr(MockRng);
    let mut dst = [0u8; 1024];
    rng.fill_bytes(&mut dst);
}

#[test]
fn test_fill_bytes_arbitrary_size() {
    struct MockRng;
    
    impl TryRngCore for MockRng {
        type Error = ();
        
        fn try_next_u32(&mut self) -> Result<u32, Self::Error> { Ok(0) }
        fn try_next_u64(&mut self) -> Result<u64, Self::Error> { Ok(0) }
        
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            for byte in dst.iter_mut() {
                *byte = 3;
            }
            Ok(())
        }
    }

    let mut rng = UnwrapErr(MockRng);
    let mut dst = [0u8; 512];
    rng.fill_bytes(&mut dst);
}

