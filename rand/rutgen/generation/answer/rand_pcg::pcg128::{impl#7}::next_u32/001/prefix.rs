// Answer 0

#[test]
fn test_next_u32_with_min_state() {
    let mut rng = Mcg128Xsl64 { state: 0 };
    let _result = rng.next_u32();
}

#[test]
fn test_next_u32_with_max_state() {
    let mut rng = Mcg128Xsl64 { state: (1u128 << 128) - 1 };
    let _result = rng.next_u32();
}

#[test]
fn test_next_u32_with_middle_state() {
    let mut rng = Mcg128Xsl64 { state: (1u128 << 127) };
    let _result = rng.next_u32();
}

#[test]
fn test_next_u32_with_random_state() {
    let mut rng = Mcg128Xsl64 { state: 123456789012345678901234567890 };
    let _result = rng.next_u32();
}

