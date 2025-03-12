// Answer 0

#[test]
fn test_extract_with_fast_prefilters() {
    use regex_syntax::hir::{Hir, HirKind, Literal};
    use crate::{MatchKind, Prefilter};

    // Constructing a HIR that satisfies the conditions
    let literal1 = Hir::literal(Literal::new("abc".into()));
    let literal2 = Hir::literal(Literal::new("def".into()));
    let literal3 = Hir::literal(Literal::new("ghi".into()));

    // Creating a top-level concatenation of HIR elements
    let concat_hir = Hir::concat(vec![literal1.clone(), literal2.clone(), literal3.clone()]);

    // Creating a slice of HIR references for the extract function
    let hirs: Vec<&Hir> = vec![&concat_hir];

    // Calling the extract function
    let result = extract(&hirs);
}

#[test]
fn test_extract_with_multiple_fast_prefilters() {
    use regex_syntax::hir::{Hir, HirKind, Literal};
    use crate::{MatchKind, Prefilter};

    // Constructing HIR that satisfies the conditions
    let literal1 = Hir::literal(Literal::new("xyz".into()));
    let literal2 = Hir::literal(Literal::new("abc".into()));
    let literal3 = Hir::literal(Literal::new("def".into()));

    // Creating a top-level concatenation of HIR elements
    let concat_hir = Hir::concat(vec![literal1.clone(), literal2.clone(), literal3.clone()]);

    // Creating a slice of HIR references for the extract function
    let hirs: Vec<&Hir> = vec![&concat_hir];

    // Calling the extract function
    let result = extract(&hirs);
}

