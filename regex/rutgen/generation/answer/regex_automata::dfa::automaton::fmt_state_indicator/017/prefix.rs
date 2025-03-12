// Answer 0

#[derive(Default)]
struct TestAutomaton {
    dead_state: bool,
    quit_state: bool,
    start_state: bool,
    match_state: bool,
    accel_state: bool,
}

impl TestAutomaton {
    fn new(dead: bool, quit: bool, start: bool, match_state: bool, accel: bool) -> Self {
        Self {
            dead_state: dead,
            quit_state: quit,
            start_state: start,
            match_state,
            accel_state: accel,
        }
    }
}

impl Automaton for TestAutomaton {
    fn is_dead_state(&self, _: StateID) -> bool {
        self.dead_state
    }

    fn is_quit_state(&self, _: StateID) -> bool {
        self.quit_state
    }

    fn is_start_state(&self, _: StateID) -> bool {
        self.start_state
    }

    fn is_match_state(&self, _: StateID) -> bool {
        self.match_state
    }

    fn is_accel_state(&self, _: StateID) -> bool {
        self.accel_state
    }
}

#[test]
fn test_fmt_state_indicator_accel_state() {
    let mut buffer = Vec::new();
    let dfa = TestAutomaton::new(false, false, false, false, true);
    let state_id = StateID::default(); // Assuming a default StateID can be used

    let result = fmt_state_indicator(&mut buffer, dfa, state_id);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&buffer), "A ");
}

#[test]
fn test_fmt_state_indicator_non_accel_state() {
    let mut buffer = Vec::new();
    let dfa = TestAutomaton::new(false, false, false, false, false);
    let state_id = StateID::default(); // Assuming a default StateID can be used

    let result = fmt_state_indicator(&mut buffer, dfa, state_id);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&buffer), "  ");
}

