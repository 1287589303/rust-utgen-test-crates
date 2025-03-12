// Answer 0

#[test]
fn test_lower_min_value() {
    let range = ClassBytesRange { start: 0, end: 100 };
    let result = range.lower();
}

#[test]
fn test_lower_middle_value() {
    let range = ClassBytesRange { start: 128, end: 200 };
    let result = range.lower();
}

#[test]
fn test_lower_max_value() {
    let range = ClassBytesRange { start: 255, end: 255 };
    let result = range.lower();
}

#[test]
fn test_lower_boundary_value() {
    let range = ClassBytesRange { start: 255, end: 0 };
    let result = range.lower();
}

#[test]
fn test_lower_zero_boundary() {
    let range = ClassBytesRange { start: 0, end: 0 };
    let result = range.lower();
}

