// Answer 0

#[test]
fn test_fmt_empty_dfa() {
    struct DummyTransitions;
    struct DummyStartTable;

    impl AsRef<[u8]> for DummyTransitions {
        fn as_ref(&self) -> &[u8] {
            &[]
        }
    }

    impl AsRef<[u8]> for DummyStartTable {
        fn as_ref(&self) -> &[u8] {
            &[]
        }
    }

    let dfa = regex_automata::dfa::SparseDFA {
        tt: DummyTransitions,
        st: DummyStartTable,
        special: Special {
            max: 0,
            quit_id: 0,
            min_match: 0,
            max_match: 0,
            min_accel: 0,
            max_accel: 0,
            min_start: 0,
            max_start: 0,
        },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags {
            has_empty: false,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };

    let result = dfa.fmt(&mut std::fmt::Formatter::new());
}

