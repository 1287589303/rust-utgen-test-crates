// Answer 0

#[test]
fn test_state_builder_matches_empty() {
    let state_builder = StateBuilderMatches(Vec::new());
    let formatter = &mut core::fmt::Formatter::default();
    state_builder.fmt(formatter);
}

#[test]
fn test_state_builder_matches_single_byte() {
    let state_builder = StateBuilderMatches(vec![0]);
    let formatter = &mut core::fmt::Formatter::default();
    state_builder.fmt(formatter);
}

#[test]
fn test_state_builder_matches_full_range() {
    let state_builder = StateBuilderMatches((0..=255).map(|b| b as u8).collect());
    let formatter = &mut core::fmt::Formatter::default();
    state_builder.fmt(formatter);
}

