// Answer 0

#[derive(Debug)]
struct NonFinite {
    value: f32,
}

impl NonFinite {
    fn to_bits(self) -> u32 {
        self.value.to_bits()
    }

    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u32 = 0x007fffff;
        const SIGN_MASK: u32 = 0x80000000;
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

#[test]
fn test_format_nonfinite_neg_infinity() {
    let non_finite = NonFinite { value: f32::NEG_INFINITY };
    let result = non_finite.format_nonfinite();
    assert_eq!(result, "NEG_INFINITY");
}

