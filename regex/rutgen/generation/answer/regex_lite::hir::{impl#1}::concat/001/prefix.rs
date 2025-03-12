// Answer 0

#[test]
fn test_concat_empty_vector() {
    let subs: Vec<Hir> = Vec::new();
    let result = Hir::concat(subs);
}

#[test]
fn test_concat_empty_vector_initialization() {
    let subs: Vec<Hir> = vec![];
    let result = Hir::concat(subs);
}

