// Answer 0

#[test]
#[should_panic]
fn test_pow5_factor_zero() {
    let value = 0;
    let _ = pow5_factor(value);
}

#[test]
fn test_pow5_factor_normal() {
    let values = [1, 2, 3, 4, 5, 10, 20, 100, 3689348814741910322];
    for &value in &values {
        let _ = pow5_factor(value);
    }
}

#[test]
fn test_pow5_factor_boundary() {
    let values = [3689348814741910323, 3689348814741910324];
    for &value in &values {
        let _ = pow5_factor(value);
    }
}

