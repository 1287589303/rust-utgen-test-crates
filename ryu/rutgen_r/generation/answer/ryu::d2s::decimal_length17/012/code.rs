// Answer 0

#[test]
fn test_decimal_length17_boundary_case() {
    let v: u64 = 100000; // Precondition: v < 100000000000000000 is true and v >= 100000 is true
    let result = decimal_length17(v);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length17_edge_case_just_below() {
    let v: u64 = 99999; // Precondition: v < 100000000000000000 is true and v < 100000 is true
    let result = decimal_length17(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length17_edge_case_just_above() {
    let v: u64 = 100001; // Precondition: v < 100000000000000000 is true and v > 100000 is true
    let result = decimal_length17(v);
    assert_eq!(result, 6);
}

