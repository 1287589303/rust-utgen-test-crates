// Answer 0

#[test]
fn test_is_i64_negint() {
    let number = Number {
        n: N::NegInt(-1),
    };
    let _ = number.is_i64();
}

#[test]
fn test_is_i64_negint_min() {
    let number = Number {
        n: N::NegInt(i64::MIN),
    };
    let _ = number.is_i64();
}

#[test]
fn test_is_i64_negint_large_negative() {
    let number = Number {
        n: N::NegInt(-100),
    };
    let _ = number.is_i64();
}

#[test]
fn test_is_i64_negint_large_negative_boundary() {
    let number = Number {
        n: N::NegInt(-i64::MAX),
    };
    let _ = number.is_i64();
}

