// Answer 0

#[test]
fn test_extract_with_one_hir_element_empty_suffix() {
    use regex_syntax::hir::{Hir, HirKind};

    let empty_hir = Hir::concat(Vec::new());
    let hirs: Vec<&Hir> = vec![&empty_hir];

    let result = extract(&hirs);
}

#[test]
fn test_extract_with_top_concat_hir_and_empty_suffix() {
    use regex_syntax::hir::{Hir, HirKind};

    let hir1 = Hir::literal("a".into());
    let hir2 = Hir::literal("b".into());
    let concat_hir = Hir::concat(vec![hir1, hir2]);
    let hirs: Vec<&Hir> = vec![&concat_hir];

    let result = extract(&hirs);
}

#[test]
fn test_extract_with_multiple_leading_empty_hirs() {
    use regex_syntax::hir::{Hir, HirKind};

    let empty_hir = Hir::literal("".into());
    let hir1 = Hir::literal("a".into());
    let concat_hir = Hir::concat(vec![empty_hir, hir1]);
    let hirs: Vec<&Hir> = vec![&concat_hir];

    let result = extract(&hirs);
}

