// Answer 0

#[test]
fn test_decimal_length17_exactly_17_digits() {
    let v: u64 = 10000000000000000;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_just_below_17_digits() {
    let v: u64 = 9999999999999999;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_exactly_16_digits() {
    let v: u64 = 1000000000000000;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_just_below_16_digits() {
    let v: u64 = 999999999999999;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_exactly_15_digits() {
    let v: u64 = 100000000000000;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_just_below_15_digits() {
    let v: u64 = 99999999999999;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_exactly_14_digits() {
    let v: u64 = 10000000000000;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_just_below_14_digits() {
    let v: u64 = 9999999999999;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_exactly_13_digits() {
    let v: u64 = 1000000000000;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_just_below_13_digits() {
    let v: u64 = 999999999999;
    let _ = decimal_length17(v);
}

