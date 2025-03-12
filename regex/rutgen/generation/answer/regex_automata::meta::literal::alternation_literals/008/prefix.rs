// Answer 0

#[test]
fn test_alternation_literals_valid_case() {
    use regex_syntax::hir::{Hir, HirKind, Literal};

    let config = Config::new()
        .match_kind(MatchKind::LeftmostFirst)
        .nfa_size_limit(None)
        .onepass_size_limit(None);

    let info = RegexInfo(Arc::new(RegexInfoI {
        config,
        props: vec![hir::Properties::new_empty()],
    }));

    let lit1 = Literal::from_bytes(b"foo");
    let lit2 = Literal::from_bytes(b"bar");

    let expr1 = Hir::from(HirKind::Literal(lit1));
    let expr2 = Hir::from(HirKind::Literal(lit2));
    let concat_expr = Hir::from(HirKind::Concat(vec![expr1.clone(), expr2.clone()]));

    let alt_hir = Hir::from(HirKind::Alternation(vec![
        concat_expr.clone(),
        concat_expr.clone(),
        concat_expr.clone(),
        concat_expr.clone(),
        concat_expr.clone(),
        concat_expr.clone(),
        concat_expr.clone(),
        concat_expr.clone(),
        concat_expr.clone(),
        concat_expr.clone(),
    ]));

    let hirs: Vec<&Hir> = vec![&alt_hir];

    let result = alternation_literals(&info, &hirs);
}

