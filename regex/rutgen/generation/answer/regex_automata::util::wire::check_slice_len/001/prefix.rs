// Answer 0

#[test]
fn test_check_slice_len_empty_slice() {
    let slice: Vec<u8> = vec![];
    let at_least_len = 1;
    let what = "empty slice";
    check_slice_len(&slice, at_least_len, what);
}

#[test]
fn test_check_slice_len_single_element_slice() {
    let slice: Vec<u8> = vec![1];
    let at_least_len = 2;
    let what = "single element slice";
    check_slice_len(&slice, at_least_len, what);
}

#[test]
fn test_check_slice_len_small_slice() {
    let slice: Vec<u8> = vec![1, 2];
    let at_least_len = 3;
    let what = "small slice";
    check_slice_len(&slice, at_least_len, what);
}

#[test]
fn test_check_slice_len_large_slice_under_bound() {
    let slice: Vec<u8> = vec![1, 2, 3, 4, 5];
    let at_least_len = 6;
    let what = "large slice under bound";
    check_slice_len(&slice, at_least_len, what);
}

#[test]
fn test_check_slice_len_boundary_case() {
    let slice: Vec<u8> = vec![1, 2, 3];
    let at_least_len = 4;
    let what = "boundary case slice";
    check_slice_len(&slice, at_least_len, what);
}

