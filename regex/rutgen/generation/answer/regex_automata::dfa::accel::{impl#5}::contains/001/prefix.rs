// Answer 0

#[test]
fn test_contains_with_existing_byte() {
    let mut accel = Accel::new();
    for byte in 1..=7 {
        accel.add(byte);
    }
    let byte_to_test = 5;
    accel.contains(byte_to_test);
}

#[test]
fn test_contains_with_non_existing_byte() {
    let mut accel = Accel::new();
    for byte in 1..=7 {
        accel.add(byte);
    }
    let byte_to_test = 8;
    accel.contains(byte_to_test);
}

#[test]
fn test_contains_on_empty_accel() {
    let accel = Accel::new();
    let byte_to_test = 0;
    accel.contains(byte_to_test);
}

#[test]
fn test_contains_with_accel_of_length_one() {
    let mut accel = Accel::new();
    accel.add(10);
    let byte_to_test = 10;
    accel.contains(byte_to_test);
}

#[test]
fn test_contains_with_accel_of_length_one_negative_case() {
    let mut accel = Accel::new();
    accel.add(10);
    let byte_to_test = 20;
    accel.contains(byte_to_test);
}

#[test]
fn test_contains_with_full_length_accel() {
    let mut accel = Accel::new();
    for byte in 1..=7 {
        accel.add(byte);
    }
    let byte_to_test = 7;
    accel.contains(byte_to_test);
} 

#[test]
fn test_contains_with_full_length_accel_negative_case() {
    let mut accel = Accel::new();
    for byte in 1..=7 {
        accel.add(byte);
    }
    let byte_to_test = 0;
    accel.contains(byte_to_test);
} 

