// Answer 0

#[test]
fn test_add_byte_less_than_capacity() {
    let mut accel = Accel::new();
    let byte = b'a';
    let _result = accel.add(byte);
}

#[test]
fn test_add_byte_with_len_two() {
    let mut accel = Accel::new();
    let _ = accel.add(b'a');
    let _ = accel.add(b'b');
    let byte = b'c';
    let _result = accel.add(byte);
}

#[test]
fn test_add_byte_with_len_two_not_contained() {
    let mut accel = Accel::new();
    let _ = accel.add(b'a');
    let _ = accel.add(b'b');
    let byte = b'd'; // not contained and not a space
    let _result = accel.add(byte);
}

#[test]
fn test_add_non_space_byte_with_len_two() {
    let mut accel = Accel::new();
    let _ = accel.add(b'a');
    let _ = accel.add(b'b');
    let byte = b'1'; // not a space and not already contained
    let _result = accel.add(byte);
}

#[test]
fn test_add_byte_first_time_not_space() {
    let mut accel = Accel::new();
    let byte = b'x'; // initially empty, this should be acceptable
    let _result = accel.add(byte);
}

