// Answer 0

#[test]
fn test_fmt_state_indicator_dead_start() {
    struct DummyAutomaton;

    impl Automaton for DummyAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            true
        }
        
        fn is_start_state(&self, _: StateID) -> bool {
            true
        }
        
        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_match_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_accel_state(&self, _: StateID) -> bool {
            false
        }
    }

    let automaton = DummyAutomaton;
    let state_id = StateID(Default::default());
    let mut output = Vec::new();
    let result = fmt_state_indicator(&mut output, automaton, state_id);
}

#[test]
fn test_fmt_state_indicator_dead_not_start() {
    struct DummyAutomaton;

    impl Automaton for DummyAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            true
        }
        
        fn is_start_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_match_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_accel_state(&self, _: StateID) -> bool {
            false
        }
    }

    let automaton = DummyAutomaton;
    let state_id = StateID(Default::default());
    let mut output = Vec::new();
    let result = fmt_state_indicator(&mut output, automaton, state_id);
} 

#[test]
fn test_fmt_state_indicator_not_dead_start() {
    struct DummyAutomaton;

    impl Automaton for DummyAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_start_state(&self, _: StateID) -> bool {
            true
        }
        
        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_match_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_accel_state(&self, _: StateID) -> bool {
            false
        }
    }

    let automaton = DummyAutomaton;
    let state_id = StateID(Default::default());
    let mut output = Vec::new();
    let result = fmt_state_indicator(&mut output, automaton, state_id);
} 

#[test]
fn test_fmt_state_indicator_dead_quit() {
    struct DummyAutomaton;

    impl Automaton for DummyAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            true
        }
        
        fn is_start_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_quit_state(&self, _: StateID) -> bool {
            true
        }
        
        fn is_match_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_accel_state(&self, _: StateID) -> bool {
            false
        }
    }

    let automaton = DummyAutomaton;
    let state_id = StateID(Default::default());
    let mut output = Vec::new();
    let result = fmt_state_indicator(&mut output, automaton, state_id);
} 

#[test]
fn test_fmt_state_indicator_alive() {
    struct DummyAutomaton;

    impl Automaton for DummyAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_start_state(&self, _: StateID) -> bool {
            true
        }
        
        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_match_state(&self, _: StateID) -> bool {
            false
        }
        
        fn is_accel_state(&self, _: StateID) -> bool {
            false
        }
    }

    let automaton = DummyAutomaton;
    let state_id = StateID(Default::default());
    let mut output = Vec::new();
    let result = fmt_state_indicator(&mut output, automaton, state_id);
} 

