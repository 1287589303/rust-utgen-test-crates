// Answer 0

#[test]
fn test_max_scalar_value_case_1() {
    let result = max_scalar_value(1);
}

#[test]
fn test_max_scalar_value_case_2() {
    let result = max_scalar_value(2);
}

#[test]
fn test_max_scalar_value_case_3() {
    let result = max_scalar_value(3);
}

#[test]
fn test_max_scalar_value_case_4() {
    let result = max_scalar_value(4);
}

#[test]
#[should_panic]
fn test_max_scalar_value_case_underflow() {
    let result = max_scalar_value(0);
}

#[test]
#[should_panic]
fn test_max_scalar_value_case_overflow() {
    let result = max_scalar_value(5);
}

