// Answer 0

#[test]
fn test_ascii_case_fold_empty_range_below() {
    let range = ClassRange { start: '!', end: '`' };
    let result = range.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_empty_range_above() {
    let range = ClassRange { start: '{', end: '~' };
    let result = range.ascii_case_fold();
}

