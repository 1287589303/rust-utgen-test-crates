// Answer 0

#[test]
fn test_check_slice_len_equal_length() {
    let slice = &[1, 2, 3]; // length is 3
    let at_least_len = 3; // setting bound to the same length
    let what = "test slice"; // descriptive string
    let _result = check_slice_len(slice, at_least_len, what);
}

#[test]
fn test_check_slice_len_zero_length() {
    let slice: &[i32] = &[]; // empty slice
    let at_least_len = 0; // setting bound to 0
    let what = "empty slice"; // descriptive string
    let _result = check_slice_len(slice, at_least_len, what);
}

#[test]
fn test_check_slice_len_large_length() {
    let slice = &[0u8; 100]; // slice of length 100
    let at_least_len = 100; // setting bound to the same length
    let what = "large slice"; // descriptive string
    let _result = check_slice_len(slice, at_least_len, what);
}

#[test]
fn test_check_slice_len_single_element() {
    let slice = &[42]; // single element slice
    let at_least_len = 1; // setting bound to the same length
    let what = "single element slice"; // descriptive string
    let _result = check_slice_len(slice, at_least_len, what);
}

