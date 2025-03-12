// Answer 0

#[test]
fn test_is_start_state_within_range() {
    struct MockSpecial {
        min_start: StateID,
        max_start: StateID,
    }

    impl MockSpecial {
        fn is_start_state(&self, id: StateID) -> bool {
            !self.is_dead_state(id) && self.min_start <= id && id <= self.max_start
        }

        fn is_dead_state(&self, id: StateID) -> bool {
            id == StateID(0)
        }
    }

    struct MockDFA {
        special: MockSpecial,
    }

    let dfa = MockDFA {
        special: MockSpecial {
            min_start: StateID(1),
            max_start: StateID(5),
        },
    };

    for id in 1..=5 {
        dfa.is_start_state(StateID(id));
    }
}

#[test]
fn test_is_start_state_dead_state() {
    struct MockSpecial {
        min_start: StateID,
        max_start: StateID,
    }

    impl MockSpecial {
        fn is_start_state(&self, id: StateID) -> bool {
            !self.is_dead_state(id) && self.min_start <= id && id <= self.max_start
        }

        fn is_dead_state(&self, id: StateID) -> bool {
            id == StateID(0)
        }
    }

    struct MockDFA {
        special: MockSpecial,
    }

    let dfa = MockDFA {
        special: MockSpecial {
            min_start: StateID(1),
            max_start: StateID(5),
        },
    };

    dfa.is_start_state(StateID(0));
}

#[test]
fn test_is_start_state_out_of_bounds() {
    struct MockSpecial {
        min_start: StateID,
        max_start: StateID,
    }

    impl MockSpecial {
        fn is_start_state(&self, id: StateID) -> bool {
            !self.is_dead_state(id) && self.min_start <= id && id <= self.max_start
        }

        fn is_dead_state(&self, id: StateID) -> bool {
            id == StateID(0)
        }
    }

    struct MockDFA {
        special: MockSpecial,
    }

    let dfa = MockDFA {
        special: MockSpecial {
            min_start: StateID(1),
            max_start: StateID(5),
        },
    };

    for id in [-1, 0, 6, 7].iter() {
        dfa.is_start_state(StateID(*id));
    }
}

