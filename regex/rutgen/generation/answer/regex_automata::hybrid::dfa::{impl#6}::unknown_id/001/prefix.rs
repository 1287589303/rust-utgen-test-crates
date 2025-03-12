// Answer 0

#[test]
fn test_unknown_id_valid() {
    struct TestDFA;
    impl TestDFA {
        fn unknown_id(&self) -> LazyStateID {
            LazyStateID::new(0).unwrap().to_unknown()
        }
    }
    let dfa = TestDFA;
    let _ = dfa.unknown_id();
}

