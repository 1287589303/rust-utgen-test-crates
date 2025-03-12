// Answer 0

#[test]
fn test_values_mut_default_i32_string() {
    let _: ValuesMut<i32, String> = ValuesMut::default();
}

#[test]
fn test_values_mut_default_empty() {
    let _: ValuesMut<(), ()> = ValuesMut::default();
}

