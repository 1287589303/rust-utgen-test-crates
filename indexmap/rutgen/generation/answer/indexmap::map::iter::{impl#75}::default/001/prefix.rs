// Answer 0

#[test]
fn test_into_values_default() {
    let default_values: IntoValues<(), ()> = IntoValues::default();
    let empty_iter = default_values.iter;
}

#[test]
fn test_into_values_default_non_empty_types() {
    let default_values: IntoValues<i32, String> = IntoValues::default();
    let empty_iter = default_values.iter;
}

