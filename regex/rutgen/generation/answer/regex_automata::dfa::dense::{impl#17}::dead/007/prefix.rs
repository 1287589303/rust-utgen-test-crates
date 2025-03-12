// Answer 0

#[test]
fn test_dead_function_exceeding_start_states() {
    #[derive(Clone)]
    struct TestLookMatcher {
        lineterm: u8,
    }

    impl LookMatcher {
        fn get_line_terminator(&self) -> u8 {
            self.lineterm
        }
    }

    let lookm = TestLookMatcher { lineterm: b'\n' };

    let kind = StartKind::Both;
    let pattern_len = Some(PatternID::LIMIT);

    let result = StartTable::<Vec<u32>>::dead(kind, &lookm, pattern_len);
}

