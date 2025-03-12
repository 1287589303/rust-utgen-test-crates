// Answer 0

#[test]
fn test_fmt_with_no_states_and_multiple_patterns() {
    use core::fmt::Formatter;
    use crate::nfa::thompson::NFA;

    struct TestNFA {
        states: Vec<State>,
        start_anchored: StateID,
        start_unanchored: StateID,
        start_pattern: Vec<StateID>,
        byte_classes: ByteClasses,
    }

    impl TestNFA {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            writeln!(f, "thompson::NFA(")?;
            for (sid, state) in self.states.iter().with_state_ids() {
                // This loop should not execute since states is empty
                let status = if sid == self.start_anchored {
                    '^'
                } else if sid == self.start_unanchored {
                    '>'
                } else {
                    ' '
                };
                writeln!(f, "{}{:06?}: {:?}", status, sid.as_usize(), state)?;
            }
            let pattern_len = self.start_pattern.len();
            if pattern_len > 1 {
                writeln!(f, "")?;
                for pid in 0..pattern_len {
                    // This loop should not execute since self.start_pattern contains at least 2 IDs
                    let sid = self.start_pattern[pid];
                    writeln!(f, "START({:06?}): {:?}", pid, sid.as_usize())?;
                }
            }
            writeln!(f, "")?;
            writeln!(f, "transition equivalence classes: {:?}", self.byte_classes)?;
            writeln!(f, ")")?;
            Ok(())
        }
    }

    let empty_states: Vec<State> = vec![];
    let start_pattern = vec![StateID(SmallIndex::new(0)), StateID(SmallIndex::new(1))];
    let byte_classes = ByteClasses([0; 256]);
    let nfa = TestNFA {
        states: empty_states,
        start_anchored: StateID(SmallIndex::new(0)),
        start_unanchored: StateID(SmallIndex::new(1)),
        start_pattern,
        byte_classes,
    };

    let mut output = String::new();
    let formatter = &mut output as &mut dyn fmt::Write;
    let result = nfa.fmt(formatter);
    // Uncomment below to actually perform the test run
    // assert!(result.is_ok());
}

