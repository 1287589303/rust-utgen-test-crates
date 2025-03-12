// Answer 0

#[test]
fn test_is_valid_with_equal_start_and_end() {
    let range = ScalarRange { start: 0, end: 0 };
    range.is_valid();
}

#[test]
fn test_is_valid_with_start_less_than_end() {
    let range = ScalarRange { start: 5, end: 10 };
    range.is_valid();
}

#[test]
fn test_is_valid_with_start_equal_to_max_u32() {
    let range = ScalarRange { start: 4294967295, end: 4294967295 };
    range.is_valid();
}

#[test]
fn test_is_valid_with_start_less_than_max_u32() {
    let range = ScalarRange { start: 4294967294, end: 4294967295 };
    range.is_valid();
}

#[test]
fn test_is_valid_with_start_greater_than_end() {
    let range = ScalarRange { start: 10, end: 5 };
    range.is_valid();
}

