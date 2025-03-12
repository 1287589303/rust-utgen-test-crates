// Answer 0

#[test]
fn test_log2_pow5_e_is_0() {
    let e = 0;
    let result = log2_pow5(e);
    assert_eq!(result, 0);
}

#[test]
fn test_log2_pow5_e_is_3528() {
    let e = 3528;
    let expected_result = ((e as u32 * 1217359) >> 19) as i32;
    let result = log2_pow5(e);
    assert_eq!(result, expected_result);
}

