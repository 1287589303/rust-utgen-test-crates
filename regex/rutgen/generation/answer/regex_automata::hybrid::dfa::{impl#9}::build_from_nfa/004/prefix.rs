// Answer 0

#[test]
fn test_build_from_nfa_insufficient_cache_capacity() {
    let mut builder = Builder::new();

    let nfa = thompson::NFA::always_match();

    let cache_capacity = 1; // intentionally set less than min_cache
    let config = Config::new()
        .quit(0, true) // setup to ensure quit_set_from_nfa returns Ok
        .cache_capacity(cache_capacity)
        .skip_cache_capacity_check(false); // skip cache capacity check should be false

    builder.configure(config);

    let result = builder.build_from_nfa(nfa);
    // The result should be an error due to insufficient cache capacity
}

#[test]
fn test_build_from_nfa_empty_nfa() {
    let mut builder = Builder::new();

    let nfa = thompson::NFA::never_match();

    let cache_capacity = 1; // intentionally set less than min_cache
    let config = Config::new()
        .quit(0, true) // setup to ensure quit_set_from_nfa returns Ok
        .cache_capacity(cache_capacity)
        .skip_cache_capacity_check(false); // skip cache capacity check should be false

    builder.configure(config);

    let result = builder.build_from_nfa(nfa);
    // The result should be an error due to insufficient cache capacity
}

#[test]
fn test_build_from_nfa_non_matching_nfa() {
    let mut builder = Builder::new();

    let nfa = thompson::NFA::config(); // configure a non-matching NFA

    let cache_capacity = 1; // intentionally set less than min_cache
    let config = Config::new()
        .quit(0, true) // setup to ensure quit_set_from_nfa returns Ok
        .cache_capacity(cache_capacity)
        .skip_cache_capacity_check(false); // skip cache capacity check should be false

    builder.configure(config);

    let result = builder.build_from_nfa(nfa);
    // The result should be an error due to insufficient cache capacity
}

