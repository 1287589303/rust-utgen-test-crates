// Answer 0

#[test]
fn test_is_ascii_valid_range_1() {
    let range = ScalarRange { start: 0, end: 127 };
    range.is_ascii();
}

#[test]
fn test_is_ascii_valid_range_2() {
    let range = ScalarRange { start: 10, end: 50 };
    range.is_ascii();
}

#[test]
fn test_is_ascii_valid_range_3() {
    let range = ScalarRange { start: 60, end: 100 };
    range.is_ascii();
}

#[test]
fn test_is_ascii_valid_range_4() {
    let range = ScalarRange { start: 0x7f, end: 0x7f };
    range.is_ascii();
}

#[test]
fn test_is_ascii_valid_range_5() {
    let range = ScalarRange { start: 1, end: 1 };
    range.is_ascii();
}

