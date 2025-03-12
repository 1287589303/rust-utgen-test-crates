// Answer 0

#[test]
fn test_log2_pow5_zero() {
    let result = log2_pow5(0);
}

#[test]
#[should_panic]
fn test_log2_pow5_over_bound() {
    let result = log2_pow5(3529); 
}

