// Answer 0

#[test]
fn test_extract_with_empty_hirs() {
    let hirs: Vec<&Hir> = vec![];
    let result = extract(&hirs);
}

#[test]
fn test_extract_with_two_hirs() {
    let hirs: Vec<&Hir> = vec![&Hir::empty(), &Hir::empty()];
    let result = extract(&hirs);
}

#[test]
fn test_extract_with_more_than_two_hirs() {
    let hirs: Vec<&Hir> = vec![&Hir::empty(), &Hir::empty(), &Hir::empty()];
    let result = extract(&hirs);
}

