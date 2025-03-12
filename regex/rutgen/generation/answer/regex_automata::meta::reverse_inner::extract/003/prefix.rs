// Answer 0

#[test]
fn test_extract_valid_case() {
    use regex_syntax::hir::{self, Hir};

    struct TestHir;

    // Construct a valid HIR that forms a top-level concatenation.
    let hirs: Vec<&Hir> = vec![&Hir::concat(vec![
        Hir::literal("fast"),
        Hir::literal("inner"),
    ])];

    // Call the extract function.
    let result = extract(&hirs);
}

#[test]
fn test_extract_with_fast_prefilter() {
    use regex_syntax::hir::{self, Hir};

    struct FastPrefilter;

    // Construct a valid HIR with at least one fast prefilter.
    let hirs: Vec<&Hir> = vec![&Hir::concat(vec![
        Hir::literal("fast"),
        Hir::literal("inner"),
    ])];

    // Call the extract function.
    let result = extract(&hirs);
}

#[test]
fn test_extract_with_non_fast_prefilter() {
    use regex_syntax::hir::{self, Hir};

    struct NonFastPrefilter;

    // Construct a valid HIR that results in a non-fast prefilter.
    let hirs: Vec<&Hir> = vec![&Hir::concat(vec![
        Hir::literal("outer"),
        Hir::literal("non_fast_inner"),
    ])];

    // Call the extract function.
    let result = extract(&hirs);
}

