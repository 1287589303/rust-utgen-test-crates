// Answer 0

#[test]
fn test_fmt_empty() {
    let pattern_epsilons = PatternEpsilons(0);
    let mut output = vec![];
    {
        let writer = &mut output;
        pattern_epsilons.fmt(writer).unwrap();
    }
}

#[test]
fn test_fmt_non_empty_with_pattern_id() {
    let pattern_id = PatternID::new_unchecked(1);
    let pattern_epsilons = PatternEpsilons(pattern_id.as_usize() << PatternEpsilons::PATTERN_ID_SHIFT);
    let mut output = vec![];
    {
        let writer = &mut output;
        pattern_epsilons.fmt(writer).unwrap();
    }
}

#[test]
fn test_fmt_non_empty_with_epsilons() {
    let epsilons_value = 5;
    let epsilons = Epsilons(epsilons_value);
    let pattern_epsilons = PatternEpsilons(epsilons_value);
    let mut output = vec![];
    {
        let writer = &mut output;
        pattern_epsilons.fmt(writer).unwrap();
    }
}

#[test]
fn test_fmt_non_empty_with_pattern_id_and_epsilons() {
    let pattern_id = PatternID::new_unchecked(2);
    let epsilons_value = 3;
    let pattern_epsilons = PatternEpsilons((pattern_id.as_usize() << PatternEpsilons::PATTERN_ID_SHIFT) | epsilons_value);
    let mut output = vec![];
    {
        let writer = &mut output;
        pattern_epsilons.fmt(writer).unwrap();
    }
}

