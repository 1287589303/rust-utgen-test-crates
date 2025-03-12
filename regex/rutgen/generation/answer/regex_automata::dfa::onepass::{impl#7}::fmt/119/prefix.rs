// Answer 0

#[test]
fn test_fmt_empty_dfa() {
    use alloc::vec::Vec;

    // Creating a dummy Config
    let config = Config::default();

    // Creating an empty NFA
    let nfa = NFA::default();

    // Creating an empty DFA
    let dfa = DFA {
        config,
        nfa,
        table: Vec::new(),
        starts: Vec::new(),
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    // Creating a mutable reference for the Formatter
    let mut output = alloc::string::String::new();
    let mut formatter = core::fmt::Formatter::new(&mut output);

    // Invoking fmt on the empty DFA
    let _ = dfa.fmt(&mut formatter);
}

