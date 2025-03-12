// Answer 0

#[test]
fn test_reset_cache_valid_dfa_with_allocated_cache() {
    use crate::hybrid::dfa::DFA;

    let dfa = DFA::always_match().unwrap();

    let mut cache = dfa.create_cache();
    cache.clear_count = 5; // Set a non-zero clear count for the test

    dfa.reset_cache(&mut cache);

    assert_eq!(cache.clear_count, 0);
}

#[test]
#[should_panic]
fn test_reset_cache_invalidates_lazy_ids() {
    use crate::hybrid::dfa::{DFA, Cache};

    let dfa = DFA::never_match().unwrap();
    let mut cache = dfa.create_cache();

    cache.starts.push(LazyStateID::new(1)); // Add a valid state ID

    dfa.reset_cache(&mut cache);

    // This should panic because the lazy state IDs should be invalid
    let _ = cache.starts[0];
}

#[test]
fn test_reset_cache_multiple_dfas() {
    use crate::hybrid::dfa::{DFA, HalfMatch, Input};

    let dfa1 = DFA::new(r"\w").unwrap();
    let dfa2 = DFA::new(r"\W").unwrap();

    let mut cache = dfa1.create_cache();
    cache.clear_count = 1;

    dfa1.reset_cache(&mut cache);
    assert_eq!(cache.clear_count, 0);

    let _ = dfa2.reset_cache(&mut cache);
}

