// Answer 0

#[test]
fn test_extract_with_non_fast_prefilter_and_multiple_concat() {
    use regex_syntax::hir::{Hir, HirKind, literal};
    use crate::{MatchKind, Prefilter};

    let literal_hir1 = Hir::literal("hello".into());
    let literal_hir2 = Hir::literal("world".into());
    
    let concat_hir = Hir::concat(vec![literal_hir1.clone(), literal_hir2.clone()]);

    let hirs = vec![&concat_hir];

    let result = extract(&hirs);
    // The result is expected to be None because prefilter for the second hir is not fast.
}

#[test]
fn test_extract_with_non_fast_prefilter_and_no_inner() {
    use regex_syntax::hir::{Hir, HirKind, literal};
    use crate::{MatchKind, Prefilter};

    let literal_hir = Hir::literal("foo".into());

    let concat_hir = Hir::concat(vec![
        literal_hir.clone(),
        Hir::literal("bar".into()),
        Hir::literal("baz".into())
    ]);

    let hirs = vec![&concat_hir];

    let result = extract(&hirs);
    // The result is expected to be None because after checking concat, it will not find a fast prefilter.
}

