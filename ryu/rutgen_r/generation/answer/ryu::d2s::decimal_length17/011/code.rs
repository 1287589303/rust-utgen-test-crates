// Answer 0

#[test]
fn test_decimal_length17_boundary_case_1() {
    let v: u64 = 1000000;
    let result = decimal_length17(v);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length17_boundary_case_2() {
    let v: u64 = 999999; 
    let result = decimal_length17(v);
    assert_eq!(result, 6);
}

