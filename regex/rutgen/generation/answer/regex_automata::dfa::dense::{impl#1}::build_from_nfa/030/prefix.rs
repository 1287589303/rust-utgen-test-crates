// Answer 0

#[test]
fn test_build_from_nfa_unicode_word_boundary_true() {
    use regex_automata::{
        dfa::{dense, Automaton},
        nfa::thompson::NFA,
        util::{alphabet::ByteSet, prefilter::Prefilter},
    };

    let nfa = NFA::new("abc").unwrap();
    let mut builder = dense::Builder::new();
    builder.configure(
        dense::Config::new()
            .unicode_word_boundary(true)
            .byte_classes(false)
            .minimize(false)
            .accelerate(false)
            .specialize_start_states(false),
    );

    let mut quitset = ByteSet::empty();
    for b in 0x80..=0xFF {
        quitset.add(b);
    }

    let dfa = builder.build_from_nfa(&nfa).unwrap();
}

#[test]
fn test_build_from_nfa_unicode_word_boundary_false() {
    use regex_automata::{
        dfa::{dense, Automaton},
        nfa::thompson::NFA,
        util::{alphabet::ByteSet, prefilter::Prefilter},
    };

    let nfa = NFA::new("abc").unwrap();
    let mut builder = dense::Builder::new();
    builder.configure(
        dense::Config::new()
            .unicode_word_boundary(false)
            .byte_classes(false)
            .minimize(false)
            .accelerate(false)
            .specialize_start_states(false),
    );

    let dfa = builder.build_from_nfa(&nfa).unwrap();
}

