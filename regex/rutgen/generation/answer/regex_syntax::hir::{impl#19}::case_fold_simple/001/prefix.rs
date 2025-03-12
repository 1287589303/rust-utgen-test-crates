// Answer 0

#[test]
fn test_case_fold_simple_below_lowercase_a() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'\0', b'@');
    range.case_fold_simple(&mut ranges).unwrap();
}

#[test]
fn test_case_fold_simple_above_uppercase_z() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'|', b'\xFF');
    range.case_fold_simple(&mut ranges).unwrap();
}

#[test]
fn test_case_fold_simple_between_uppercase_and_lowercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'[', b'`');
    range.case_fold_simple(&mut ranges).unwrap();
}

#[test]
fn test_case_fold_simple_empty_range() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(b'Z', b'Z');
    range.case_fold_simple(&mut ranges).unwrap();
}

