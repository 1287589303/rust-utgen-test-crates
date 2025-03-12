// Answer 0

#[test]
fn test_decimal_length17_min_value() {
    assert_eq!(decimal_length17(0), 1);
}

#[test]
fn test_decimal_length17_single_digit() {
    assert_eq!(decimal_length17(5), 1);
}

#[test]
fn test_decimal_length17_double_digit() {
    assert_eq!(decimal_length17(50), 2);
}

#[test]
fn test_decimal_length17_triple_digit() {
    assert_eq!(decimal_length17(500), 3);
}

#[test]
fn test_decimal_length17_quadruple_digit() {
    assert_eq!(decimal_length17(5000), 4);
}

#[test]
fn test_decimal_length17_quintuple_digit() {
    assert_eq!(decimal_length17(50000), 5);
}

#[test]
fn test_decimal_length17_sextuple_digit() {
    assert_eq!(decimal_length17(500000), 6);
}

#[test]
fn test_decimal_length17_septuple_digit() {
    assert_eq!(decimal_length17(5000000), 7);
}

#[test]
fn test_decimal_length17_octuple_digit() {
    assert_eq!(decimal_length17(50000000), 8);
}

#[test]
fn test_decimal_length17_nontuple_digit() {
    assert_eq!(decimal_length17(500000000), 9);
}

#[test]
fn test_decimal_length17_decuple_digit() {
    assert_eq!(decimal_length17(5000000000), 10);
}

#[test]
fn test_decimal_length17_undecuple_digit() {
    assert_eq!(decimal_length17(50000000000), 11);
}

#[test]
fn test_decimal_length17_duodecuple_digit() {
    assert_eq!(decimal_length17(500000000000), 12);
}

#[test]
fn test_decimal_length17_tredecuple_digit() {
    assert_eq!(decimal_length17(5000000000000), 13);
}

#[test]
fn test_decimal_length17_quattuordecuple_digit() {
    assert_eq!(decimal_length17(50000000000000), 14);
}

#[test]
fn test_decimal_length17_quindecuple_digit() {
    assert_eq!(decimal_length17(500000000000000), 15);
}

#[test]
fn test_decimal_length17_digit_boundary() {
    assert_eq!(decimal_length17(10000000000000000), 17);
}

#[test]
#[should_panic]
fn test_decimal_length17_above_boundary() {
    let _ = decimal_length17(100000000000000000);
}

