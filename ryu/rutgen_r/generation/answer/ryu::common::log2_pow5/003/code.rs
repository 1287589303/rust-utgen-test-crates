// Answer 0

#[test]
#[should_panic]
fn test_log2_pow5_negative_input() {
    let e: i32 = -1;
    let result = log2_pow5(e);
}

#[test]
#[should_panic]
fn test_log2_pow5_negative_large_input() {
    let e: i32 = -1000;
    let result = log2_pow5(e);
}

