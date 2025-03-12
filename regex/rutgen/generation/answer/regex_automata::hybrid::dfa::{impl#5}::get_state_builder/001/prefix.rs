// Answer 0

#[test]
fn test_get_state_builder_empty_cache() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::new(),
        next: ActiveStates::new(),
    };
    
    let dfa = DFA {
        tt: Transitions::new(),
        st: StartTable::new(),
        special: Special::new(),
        pre: None,
        quitset: ByteSet::new(),
        flags: Flags::new(),
    };

    let mut lazy = Lazy {
        dfa: &dfa,
        cache: &mut cache,
    };

    let builder = lazy.get_state_builder();
}

#[test]
fn test_get_state_builder_with_capacity() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::new(),
        next: ActiveStates::new(),
    };

    let dfa = DFA {
        tt: Transitions::new(),
        st: StartTable::new(),
        special: Special::new(),
        pre: None,
        quitset: ByteSet::new(),
        flags: Flags::new(),
    };

    let mut lazy = Lazy {
        dfa: &dfa,
        cache: &mut cache,
    };

    cache.scratch_state_builder = StateBuilderEmpty::new();

    let builder = lazy.get_state_builder();
}

#[test]
fn test_get_state_builder_non_empty_cache() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::new(),
        next: ActiveStates::new(),
    };

    let dfa = DFA {
        tt: Transitions::new(),
        st: StartTable::new(),
        special: Special::new(),
        pre: None,
        quitset: ByteSet::new(),
        flags: Flags::new(),
    };

    let mut lazy = Lazy {
        dfa: &dfa,
        cache: &mut cache,
    };

    cache.scratch_state_builder = StateBuilderEmpty::new();

    let builder = lazy.get_state_builder();
    let new_builder = StateBuilderEmpty::new();
    
    lazy.cache.scratch_state_builder = new_builder;
}

#[test]
fn test_get_state_builder_after_reset() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::new(),
        next: ActiveStates::new(),
    };

    let dfa = DFA {
        tt: Transitions::new(),
        st: StartTable::new(),
        special: Special::new(),
        pre: None,
        quitset: ByteSet::new(),
        flags: Flags::new(),
    };

    let mut lazy = Lazy {
        dfa: &dfa,
        cache: &mut cache,
    };

    let builder = lazy.get_state_builder();
    
    lazy.cache.scratch_state_builder.clear();

    let new_builder = lazy.get_state_builder();
}

