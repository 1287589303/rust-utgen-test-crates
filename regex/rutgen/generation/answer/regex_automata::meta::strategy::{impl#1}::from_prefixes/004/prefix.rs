// Answer 0

#[test]
fn test_from_prefixes_valid_case() {
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let regex_info = RegexInfo::new(config, &[&Hir::literal("test")]);
    let prefixes = literal::Seq::from_iter(vec![b"test".as_ref()]);

    let result = Pre::from_prefixes(&regex_info, &prefixes);
}

#[test]
fn test_from_prefixes_single_pattern_no_captures() {
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let group_info = GroupInfo::default();
    let regex_info = RegexInfo(Arc::new(RegexInfoI {
        config,
        props: vec![hir::Properties::default().with_explicit_captures_len(0)],
        ..Default::default()
    }));
    let prefixes = literal::Seq::from_iter(vec![b"exact".as_ref()]);

    let result = Pre::from_prefixes(&regex_info, &prefixes);
}

#[test]
fn test_from_prefixes_no_look_around() {
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let regex_info = RegexInfo::new(config, &[&Hir::literal("foo")]);
    let prefixes = literal::Seq::from_iter(vec![b"foo".as_ref()]);

    let result = Pre::from_prefixes(&regex_info, &prefixes);
}

#[test]
fn test_from_prefixes_with_aho_corasick_choice() {
    let config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let regex_info = RegexInfo::new(config, &[&Hir::literal("bar")]);
    let prefixes = literal::Seq::from_iter(vec![b"bar".as_ref()]);

    let result = Pre::from_prefixes(&regex_info, &prefixes);
}

