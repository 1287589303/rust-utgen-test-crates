// Answer 0

#[test]
fn test_split_case_start_bound() {
    let scalar_range = ScalarRange { start: 0xE000, end: 0xD7FF };
    let _result = scalar_range.split();
}

#[test]
fn test_split_case_start_bound_valid_end() {
    let scalar_range = ScalarRange { start: 0xE000, end: 0x10FFFF };
    let _result = scalar_range.split();
}

#[test]
fn test_split_case_start_bound_invalid_end() {
    let scalar_range = ScalarRange { start: 0xE000, end: 0xD7FF };
    let _result = scalar_range.split();
}

