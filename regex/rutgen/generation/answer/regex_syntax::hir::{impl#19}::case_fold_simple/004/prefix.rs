// Answer 0

#[test]
fn test_case_fold_simple_overlap_full_range() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(65, 122);
    range.case_fold_simple(&mut ranges).unwrap();
}

#[test]
fn test_case_fold_simple_overlap_lowercase_only() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(97, 122);
    range.case_fold_simple(&mut ranges).unwrap();
}

#[test]
fn test_case_fold_simple_overlap_uppercase_only() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(65, 90);
    range.case_fold_simple(&mut ranges).unwrap();
}

#[test]
fn test_case_fold_simple_overlap_single_lowercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(98, 98); // 'b'
    range.case_fold_simple(&mut ranges).unwrap();
}

#[test]
fn test_case_fold_simple_overlap_single_uppercase() {
    let mut ranges = Vec::new();
    let range = ClassBytesRange::new(67, 67); // 'C'
    range.case_fold_simple(&mut ranges).unwrap();
}

