// Answer 0

#[test]
fn test_set_universal_starts_unanchored_fail_word_byte() {
    struct TestDFA {
        st: StartKind,
    }

    impl TestDFA {
        fn start(&self, _anchored: Anchored, start: Start) -> Result<StateID, StartError> {
            match start {
                Start::NonWordByte => Ok(StateID(0)),
                Start::WordByte => Err(StartError),
                Start::Text => Ok(StateID(0)),
                Start::LineLF => Ok(StateID(0)),
                Start::LineCR => Ok(StateID(0)),
                Start::CustomLineTerminator => Ok(StateID(0)),
            }
        }
        fn start_kind(&self) -> &StartKind {
            &self.st
        }

        fn set_universal_starts(&mut self) {
            assert_eq!(6, Start::len(), "expected 6 start configurations");

            let start_id = |dfa: &mut TestDFA,
                            anchored: Anchored,
                            start: Start| {
                dfa.start(anchored, start).expect("valid Input configuration")
            };

            if self.start_kind().has_unanchored() {
                let anchor = Anchored::No;
                let sid = start_id(self, anchor, Start::NonWordByte);
                if sid == start_id(self, anchor, Start::WordByte)
                    && sid == start_id(self, anchor, Start::Text)
                    && sid == start_id(self, anchor, Start::LineLF)
                    && sid == start_id(self, anchor, Start::LineCR)
                    && sid == start_id(self, anchor, Start::CustomLineTerminator)
                {
                    // set start state logic
                }
            }
        }
    }

    let mut dfa = TestDFA { st: StartKind::Both };
    dfa.set_universal_starts();
}

#[test]
fn test_set_universal_starts_anchored_fail_word_byte() {
    struct TestDFA {
        st: StartKind,
    }

    impl TestDFA {
        fn start(&self, _anchored: Anchored, start: Start) -> Result<StateID, StartError> {
            match start {
                Start::NonWordByte => Ok(StateID(0)),
                Start::WordByte => Err(StartError),
                Start::Text => Ok(StateID(0)),
                Start::LineLF => Ok(StateID(0)),
                Start::LineCR => Ok(StateID(0)),
                Start::CustomLineTerminator => Ok(StateID(0)),
            }
        }
        fn start_kind(&self) -> &StartKind {
            &self.st
        }

        fn set_universal_starts(&mut self) {
            assert_eq!(6, Start::len(), "expected 6 start configurations");

            let start_id = |dfa: &mut TestDFA,
                            anchored: Anchored,
                            start: Start| {
                dfa.start(anchored, start).expect("valid Input configuration")
            };

            if self.start_kind().has_anchored() {
                let anchor = Anchored::Yes;
                let sid = start_id(self, anchor, Start::NonWordByte);
                if sid == start_id(self, anchor, Start::WordByte)
                    && sid == start_id(self, anchor, Start::Text)
                    && sid == start_id(self, anchor, Start::LineLF)
                    && sid == start_id(self, anchor, Start::LineCR)
                    && sid == start_id(self, anchor, Start::CustomLineTerminator)
                {
                    // set start state logic
                }
            }
        }
    }

    let mut dfa = TestDFA { st: StartKind::Both };
    dfa.set_universal_starts();
}

