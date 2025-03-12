// Answer 0

#[test]
fn test_suffixes_empty_hirs_all() {
    let kind = MatchKind::All;
    let hirs: Vec<()> = Vec::new(); // no Borrow<Hir> implementations available
    let result = suffixes(kind, &hirs);
}

#[test]
fn test_suffixes_non_hir_borrow_all() {
    struct NonHir;

    let kind = MatchKind::All;
    let hirs: Vec<NonHir> = vec![NonHir, NonHir]; // no Borrow<Hir> implementations available
    let result = suffixes(kind, &hirs);
}

