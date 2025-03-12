// Answer 0

#[test]
fn test_ascii_case_fold_lowercase() {
    let input = ClassRange { start: 'a', end: 'c' };
    let result = input.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_uppercase() {
    let input = ClassRange { start: 'A', end: 'C' };
    let result = input.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_overlap_lowercase() {
    let input = ClassRange { start: 'b', end: 'y' };
    let result = input.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_overlap_uppercase() {
    let input = ClassRange { start: 'B', end: 'Y' };
    let result = input.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_full_lowercase() {
    let input = ClassRange { start: 'a', end: 'z' };
    let result = input.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_full_uppercase() {
    let input = ClassRange { start: 'A', end: 'Z' };
    let result = input.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_whole_range() {
    let input = ClassRange { start: 'A', end: 'z' };
    let result = input.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_no_overlap() {
    let input = ClassRange { start: '!', end: '%' };
    let result = input.ascii_case_fold();
}

