// Answer 0

#[test]
fn test_prefilter_with_specialize_start_states_true() {
    use crate::util::prefilter::Prefilter;
    use crate::util::search::MatchKind;

    let pre = Prefilter {
        pre: None,
        is_fast: true,
        max_needle_len: 3,
    };

    let config = Config::new()
        .prefilter(Some(pre))
        .specialize_start_states(true)
        .match_kind(MatchKind::LeftmostFirst)
        .byte_classes(true);

    let _result = config.prefilter(Some(pre));
}

#[test]
fn test_prefilter_with_specialize_start_states_true_byte_classes_false() {
    use crate::util::prefilter::Prefilter;
    use crate::util::search::MatchKind;

    let pre = Prefilter {
        pre: None,
        is_fast: true,
        max_needle_len: 3,
    };

    let config = Config::new()
        .prefilter(Some(pre))
        .specialize_start_states(true)
        .match_kind(MatchKind::LeftmostFirst)
        .byte_classes(false);

    let _result = config.prefilter(Some(pre));
}

#[test]
fn test_prefilter_with_specialize_start_states_true_different_match_kind() {
    use crate::util::prefilter::Prefilter;
    use crate::util::search::MatchKind;

    let pre = Prefilter {
        pre: None,
        is_fast: true,
        max_needle_len: 5,
    };

    let config = Config::new()
        .prefilter(Some(pre))
        .specialize_start_states(true)
        .match_kind(MatchKind::All)
        .byte_classes(true);

    let _result = config.prefilter(Some(pre));
}

