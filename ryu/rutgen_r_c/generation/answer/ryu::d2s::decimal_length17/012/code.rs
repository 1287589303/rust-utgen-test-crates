// Answer 0

#[test]
fn test_decimal_length17_value_100000() {
    let v: u64 = 100000;
    let result = decimal_length17(v);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length17_value_99999() {
    let v: u64 = 99999; 
    let result = decimal_length17(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length17_value_999999() {
    let v: u64 = 999999; 
    let result = decimal_length17(v);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length17_value_99999999999999999() {
    let v: u64 = 99999999999999999; 
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

