// Answer 0

#[test]
fn test_posix_class_space() {
    let result = posix_class("space");
    let _ = result.unwrap().collect::<Vec<hir::ClassRange>>(); // Collect to ensure the iterator is processed
}

#[test]
fn test_posix_class_unrecognized() {
    let kinds = ["alnum", "alpha", "ascii", "blank", "cntrl", "digit", "graph", "lower", "print", "punct"];
    for &kind in kinds.iter() {
        let result = posix_class(kind);
        let _ = result.unwrap_err();
    }
}

