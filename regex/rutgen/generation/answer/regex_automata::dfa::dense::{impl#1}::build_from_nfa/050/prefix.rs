// Answer 0

#[test]
fn test_build_dfa_from_nfa_unicode_word_boundary_enabled() {
    use regex_automata::{
        dfa::{dense, Automaton},
        nfa::thompson::{self, NFA},
        util::search::MatchKind,
    };

    let mut builder = dense::Builder::new();
    builder.configure(dense::Config::new()
        .match_kind(MatchKind::LeftmostFirst)
        .accelerate(false)
        .minimize(false)
        .specialize_start_states(false)
        .starts_for_each_pattern(true)
        .byte_classes(true)
        .quit(0x81, true) // Add a quit byte (not in 0x80..=0xFF range)
        .unicode_word_boundary(true));
    
    let nfa = NFA::compiler()
        .build(r"[\w]+").unwrap();
    
    let dfa = builder.build_from_nfa(&nfa).unwrap();
} 

#[test]
fn test_build_dfa_from_nfa_with_non_empty_quitset() {
    use regex_automata::{
        dfa::{dense, Automaton},
        nfa::thompson::{self, NFA},
        util::search::MatchKind,
    };

    let mut builder = dense::Builder::new();
    builder.configure(dense::Config::new()
        .match_kind(MatchKind::LeftmostFirst)
        .accelerate(false)
        .minimize(false)
        .specialize_start_states(false)
        .starts_for_each_pattern(true)
        .byte_classes(true)
        .quit(0x81, true) // Adding a byte to the quit set
        .unicode_word_boundary(true));
    
    let nfa = NFA::compiler()
        .build(r"[^\x00-\x7F]+").unwrap(); // Ensure NFA can recognize non-ASCII characters

    let dfa = builder.build_from_nfa(&nfa).unwrap();
}

