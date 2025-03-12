// Answer 0

#[test]
fn test_alternation_literals_with_hirs_length_not_one() {
    use regex_syntax::hir::{HirKind, Literal};
    use std::sync::Arc;

    let empty_props = vec![];
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let info = RegexInfo(Arc::new(RegexInfoI {
        props: empty_props,
        config,
    }));

    let hirs: Vec<&Hir> = vec![&Hir::new(HirKind::Alternation(vec![]))];

    let result = alternation_literals(&info, &hirs);
}

#[test]
fn test_alternation_literals_with_look_set_not_empty() {
    use regex_syntax::hir::{HirKind, Literal};
    use std::sync::Arc;

    let empty_props = vec![hir::Properties::default()];
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let info = RegexInfo(Arc::new(RegexInfoI {
        props: empty_props,
        config,
    }));

    let hirs: Vec<&Hir> = vec![&Hir::new(HirKind::Alternation(vec![]))];

    let result = alternation_literals(&info, &hirs);
}

#[test]
fn test_alternation_literals_with_explicit_captures_len_greater_than_zero() {
    use regex_syntax::hir::{HirKind, Literal};
    use std::sync::Arc;

    let empty_props = vec![hir::Properties { explicit_captures_len: 1, ..Default::default() }];
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let info = RegexInfo(Arc::new(RegexInfoI {
        props: empty_props,
        config,
    }));

    let hirs: Vec<&Hir> = vec![&Hir::new(HirKind::Alternation(vec![]))];

    let result = alternation_literals(&info, &hirs);
}

#[test]
fn test_alternation_literals_with_non_alternation_literal() {
    use regex_syntax::hir::{HirKind, Literal};
    use std::sync::Arc;

    let empty_props = vec![hir::Properties::default()];
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let info = RegexInfo(Arc::new(RegexInfoI {
        props: empty_props,
        config,
    }));

    let hirs: Vec<&Hir> = vec![&Hir::new(HirKind::Concat(vec![Hir::new(HirKind::Literal(Literal(vec![b'a'])))])))];

    let result = alternation_literals(&info, &hirs);
}

