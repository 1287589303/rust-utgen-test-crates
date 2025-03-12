// Answer 0

#[test]
fn test_format_nonfinite_negative_infinity() {
    struct TestF32(f32);

    impl TestF32 {
        fn to_bits(self) -> u64 {
            self.0.to_bits() as u64
        }

        fn format_nonfinite(self) -> &'static str {
            const MANTISSA_MASK: u64 = 0x000fffffffffffff;
            const SIGN_MASK: u64 = 0x8000000000000000;
            let bits = self.to_bits();
            if bits & MANTISSA_MASK != 0 {
                "NAN"
            } else if bits & SIGN_MASK != 0 {
                "NEG_INFINITY"
            } else {
                "INFINITY"
            }
        }
    }

    let negative_infinity = TestF32(f32::NEG_INFINITY);
    let result = negative_infinity.format_nonfinite();
    assert_eq!(result, "NEG_INFINITY");
}

