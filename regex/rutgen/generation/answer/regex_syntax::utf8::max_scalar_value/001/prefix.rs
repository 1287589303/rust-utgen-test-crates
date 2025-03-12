// Answer 0

#[test]
fn test_max_scalar_value_1() {
    let result = max_scalar_value(1);
}

#[test]
fn test_max_scalar_value_2() {
    let result = max_scalar_value(2);
}

#[test]
fn test_max_scalar_value_3() {
    let result = max_scalar_value(3);
}

#[test]
fn test_max_scalar_value_4() {
    let result = max_scalar_value(4);
}

#[should_panic]
#[test]
fn test_max_scalar_value_unreachable() {
    let result = max_scalar_value(5);
}

