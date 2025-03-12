// Answer 0

#[test]
fn test_random_bool_lower_boundary() {
    let result = rand::random_bool(0.0);
}

#[test]
fn test_random_bool_upper_boundary() {
    let result = rand::random_bool(1.0);
}

#[test]
#[should_panic]
fn test_random_bool_below_minimum() {
    let result = rand::random_bool(-0.1);
}

#[test]
#[should_panic]
fn test_random_bool_above_maximum() {
    let result = rand::random_bool(1.1);
}

