// Answer 0

#[test]
fn test_ascii_case_fold_empty_class() {
    let mut class = Class::new(vec![]);
    class.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_single_range() {
    let mut class = Class::new(vec![ClassRange { start: 'A', end: 'A' }]);
    class.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_multiple_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'c' },
        ClassRange { start: 'A', end: 'C' },
    ]);
    class.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_no_case_variants() {
    let mut class = Class::new(vec![ClassRange { start: '1', end: '1' }]);
    class.ascii_case_fold();
}

#[test]
fn test_ascii_case_fold_non_contiguous_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: '0', end: '0' },
        ClassRange { start: 'a', end: 'b' },
    ]);
    class.ascii_case_fold();
}

