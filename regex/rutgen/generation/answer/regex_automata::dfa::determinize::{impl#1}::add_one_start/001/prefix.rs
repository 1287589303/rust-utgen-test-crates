// Answer 0

#[test]
fn test_add_one_start_non_word_byte() {
    let nfa_start = StateID::default();
    let start = Start::NonWordByte;
    let mut runner = Runner {
        config: Config::default(),
        nfa: &thompson::NFA::default(),
        dfa: &mut dense::OwnedDFA::default(),
        builder_states: Vec::new(),
        cache: StateMap::new(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::new(),
    };
    
    let _ = runner.add_one_start(nfa_start, start);
}

#[test]
fn test_add_one_start_word_byte() {
    let nfa_start = StateID::default();
    let start = Start::WordByte;
    let mut runner = Runner {
        config: Config::default(),
        nfa: &thompson::NFA::default(),
        dfa: &mut dense::OwnedDFA::default(),
        builder_states: Vec::new(),
        cache: StateMap::new(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::new(),
    };

    let _ = runner.add_one_start(nfa_start, start);
}

#[test]
fn test_add_one_start_text() {
    let nfa_start = StateID::default();
    let start = Start::Text;
    let mut runner = Runner {
        config: Config::default(),
        nfa: &thompson::NFA::default(),
        dfa: &mut dense::OwnedDFA::default(),
        builder_states: Vec::new(),
        cache: StateMap::new(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::new(),
    };

    let _ = runner.add_one_start(nfa_start, start);
}

#[test]
fn test_add_one_start_line_lf() {
    let nfa_start = StateID::default();
    let start = Start::LineLF;
    let mut runner = Runner {
        config: Config::default(),
        nfa: &thompson::NFA::default(),
        dfa: &mut dense::OwnedDFA::default(),
        builder_states: Vec::new(),
        cache: StateMap::new(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::new(),
    };

    let _ = runner.add_one_start(nfa_start, start);
}

#[test]
fn test_add_one_start_line_cr() {
    let nfa_start = StateID::default();
    let start = Start::LineCR;
    let mut runner = Runner {
        config: Config::default(),
        nfa: &thompson::NFA::default(),
        dfa: &mut dense::OwnedDFA::default(),
        builder_states: Vec::new(),
        cache: StateMap::new(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::new(),
    };

    let _ = runner.add_one_start(nfa_start, start);
}

#[test]
fn test_add_one_start_custom_line_terminator() {
    let nfa_start = StateID::default();
    let start = Start::CustomLineTerminator;
    let mut runner = Runner {
        config: Config::default(),
        nfa: &thompson::NFA::default(),
        dfa: &mut dense::OwnedDFA::default(),
        builder_states: Vec::new(),
        cache: StateMap::new(),
        memory_usage_state: 0,
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::new(),
    };

    let _ = runner.add_one_start(nfa_start, start);
}

