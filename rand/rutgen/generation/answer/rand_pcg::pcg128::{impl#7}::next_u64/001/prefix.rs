// Answer 0

#[test]
fn test_next_u64_zero_state() {
    let mut rng = Mcg128Xsl64 { state: 0 };
    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_max_state() {
    let mut rng = Mcg128Xsl64 { state: u128::MAX };
    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_random_state_1() {
    let mut rng = Mcg128Xsl64 { state: 1 };
    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_random_state_2() {
    let mut rng = Mcg128Xsl64 { state: 12345678901234567890 };
    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_random_state_3() {
    let mut rng = Mcg128Xsl64 { state: 98765432109876543210 };
    let _result = rng.next_u64();
}

