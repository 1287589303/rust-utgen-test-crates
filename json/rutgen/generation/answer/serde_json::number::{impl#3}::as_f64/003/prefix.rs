// Answer 0

#[test]
fn test_as_f64_positive_integer_0() {
    let number = Number {
        n: N::PosInt(0),
    };
    let _result = number.as_f64();
}

#[test]
fn test_as_f64_positive_integer_1() {
    let number = Number {
        n: N::PosInt(1),
    };
    let _result = number.as_f64();
}

#[test]
fn test_as_f64_positive_integer_42() {
    let number = Number {
        n: N::PosInt(42),
    };
    let _result = number.as_f64();
}

#[test]
fn test_as_f64_positive_integer_max() {
    let number = Number {
        n: N::PosInt(u64::MAX),
    };
    let _result = number.as_f64();
}

