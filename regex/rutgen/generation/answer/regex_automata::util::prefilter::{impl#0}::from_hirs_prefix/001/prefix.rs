// Answer 0

#[test]
fn test_from_hirs_prefix_all() {
    use regex_syntax::hir::Hir;
    use regex_automata::{util::prefilter::Prefilter, MatchKind};
    
    let hirs: Vec<Hir> = vec![
        regex_syntax::parse::parse(r"(Bruce|Patti)").unwrap(),
        regex_syntax::parse::parse(r"Mrs?\. Doubtfire").unwrap(),
    ];
    let pre = Prefilter::from_hirs_prefix(MatchKind::All, &hirs).expect("a prefilter");
}

#[test]
fn test_from_hirs_prefix_leftmost_first() {
    use regex_syntax::hir::Hir;
    use regex_automata::{util::prefilter::Prefilter, MatchKind};
    
    let hirs: Vec<Hir> = vec![
        regex_syntax::parse::parse(r"Hello \w+").unwrap(),
        regex_syntax::parse::parse(r"World").unwrap(),
    ];
    let pre = Prefilter::from_hirs_prefix(MatchKind::LeftmostFirst, &hirs).expect("a prefilter");
}

#[test]
fn test_from_hirs_prefix_empty_hir_vec() {
    use regex_automata::{util::prefilter::Prefilter, MatchKind};
    
    let hirs: Vec<&Hir> = Vec::new();
    let pre = Prefilter::from_hirs_prefix(MatchKind::All, &hirs);
    assert!(pre.is_none());
}

#[test]
fn test_from_hirs_prefix_single_hir() {
    use regex_syntax::hir::Hir;
    use regex_automata::{util::prefilter::Prefilter, MatchKind};
    
    let hirs: Vec<Hir> = vec![
        regex_syntax::parse::parse(r"Test").unwrap(),
    ];
    let pre = Prefilter::from_hirs_prefix(MatchKind::LeftmostFirst, &hirs).expect("a prefilter");
}

