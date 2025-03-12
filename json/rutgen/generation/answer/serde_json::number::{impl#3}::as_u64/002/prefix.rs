// Answer 0

#[test]
fn test_as_u64_with_neg_int() {
    let number = Number {
        n: N::NegInt(-1),
    };
    let _ = number.as_u64();
}

#[test]
fn test_as_u64_with_neg_int_large() {
    let number = Number {
        n: N::NegInt(-1000),
    };
    let _ = number.as_u64();
}

#[test]
fn test_as_u64_with_neg_int_min() {
    let number = Number {
        n: N::NegInt(i64::MIN),
    };
    let _ = number.as_u64();
}

#[test]
fn test_as_u64_with_neg_int_boundary() {
    let number = Number {
        n: N::NegInt(-2),
    };
    let _ = number.as_u64();
}

