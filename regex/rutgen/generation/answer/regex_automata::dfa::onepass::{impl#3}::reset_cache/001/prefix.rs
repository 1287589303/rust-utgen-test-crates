// Answer 0

#[test]
fn test_reset_cache_valid_dfa_non_zero_cache() {
    use regex_automata::{dfa::onepass::DFA, Cache};

    let re = DFA::new(r"\w").unwrap();
    let mut cache = re.create_cache();
    assert!(cache.explicit_slot_len > 0);
    re.reset_cache(&mut cache);
}

#[test]
fn test_reset_cache_multiple_dfa_non_zero_cache() {
    use regex_automata::{dfa::onepass::DFA, Cache};

    let re1 = DFA::new(r"\w").unwrap();
    let re2 = DFA::new(r"\W").unwrap();
    let mut cache = re1.create_cache();
    assert!(cache.explicit_slot_len > 0);
    re1.reset_cache(&mut cache);
    re2.reset_cache(&mut cache);
}

#[test]
fn test_reset_cache_with_cache_reuse() {
    use regex_automata::{dfa::onepass::DFA, Cache, Match};

    let re1 = DFA::new(r"\w").unwrap();
    let re2 = DFA::new(r"\W").unwrap();
    let mut caps1 = re1.create_captures();
    let mut caps2 = re2.create_captures();
    
    let mut cache = re1.create_cache();
    assert!(cache.explicit_slot_len > 0);
    
    re1.reset_cache(&mut cache);
    assert_eq!(
        Some(Match::must(0, 0..2)),
        { re1.captures(&mut cache, "Δ", &mut caps1); caps1.get_match() }
    );

    re2.reset_cache(&mut cache);
    assert_eq!(
        Some(Match::must(0, 0..3)),
        { re2.captures(&mut cache, "☃", &mut caps2); caps2.get_match() }
    );
}

