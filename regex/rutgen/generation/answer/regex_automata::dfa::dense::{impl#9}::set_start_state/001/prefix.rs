// Answer 0

#[test]
fn test_set_start_state_non_word_byte_no() {
    struct TestDFA {
        tt: TestTT,
        st: TestST,
    }

    struct TestTT;

    impl TestTT {
        fn is_valid(&self, _id: StateID) -> bool {
            true
        }
    }

    struct TestST;

    impl TestST {
        fn set_start(&mut self, _anchored: Anchored, _start: Start, _id: StateID) {}
    }

    let mut dfa = TestDFA {
        tt: TestTT,
        st: TestST,
    };
    let id = StateID(0);
    dfa.set_start_state(Anchored::No, Start::NonWordByte, id);
}

#[test]
fn test_set_start_state_word_byte_yes() {
    struct TestDFA {
        tt: TestTT,
        st: TestST,
    }

    struct TestTT;

    impl TestTT {
        fn is_valid(&self, _id: StateID) -> bool {
            true
        }
    }

    struct TestST;

    impl TestST {
        fn set_start(&mut self, _anchored: Anchored, _start: Start, _id: StateID) {}
    }

    let mut dfa = TestDFA {
        tt: TestTT,
        st: TestST,
    };
    let id = StateID(1);
    dfa.set_start_state(Anchored::Yes, Start::WordByte, id);
}

#[test]
fn test_set_start_state_text_pattern() {
    struct TestDFA {
        tt: TestTT,
        st: TestST,
    }

    struct TestTT;

    impl TestTT {
        fn is_valid(&self, _id: StateID) -> bool {
            true
        }
    }

    struct TestST;

    impl TestST {
        fn set_start(&mut self, _anchored: Anchored, _start: Start, _id: StateID) {}
    }

    let mut dfa = TestDFA {
        tt: TestTT,
        st: TestST,
    };
    let id = StateID(2);
    dfa.set_start_state(Anchored::Pattern(PatternID(0)), Start::Text, id);
}

#[test]
fn test_set_start_state_line_cr_custom() {
    struct TestDFA {
        tt: TestTT,
        st: TestST,
    }

    struct TestTT;

    impl TestTT {
        fn is_valid(&self, _id: StateID) -> bool {
            true
        }
    }

    struct TestST;

    impl TestST {
        fn set_start(&mut self, _anchored: Anchored, _start: Start, _id: StateID) {}
    }

    let mut dfa = TestDFA {
        tt: TestTT,
        st: TestST,
    };
    let id = StateID(3);
    dfa.set_start_state(Anchored::Yes, Start::LineCR, id);
}

#[test]
fn test_set_start_state_empty_custom() {
    struct TestDFA {
        tt: TestTT,
        st: TestST,
    }

    struct TestTT;

    impl TestTT {
        fn is_valid(&self, _id: StateID) -> bool {
            true
        }
    }

    struct TestST;

    impl TestST {
        fn set_start(&mut self, _anchored: Anchored, _start: Start, _id: StateID) {}
    }

    let mut dfa = TestDFA {
        tt: TestTT,
        st: TestST,
    };
    let id = StateID(4);
    dfa.set_start_state(Anchored::Pattern(PatternID(1)), Start::CustomLineTerminator, id);
}

