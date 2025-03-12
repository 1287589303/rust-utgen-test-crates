// Answer 0

#[test]
fn test_from_nfa_all_flags_true() {
    struct MockNFA {
        empty: bool,
        utf8: bool,
        always_anchored: bool,
    }

    impl MockNFA {
        fn has_empty(&self) -> bool {
            self.empty
        }
        
        fn is_utf8(&self) -> bool {
            self.utf8
        }
        
        fn is_always_start_anchored(&self) -> bool {
            self.always_anchored
        }
    }

    let nfa = MockNFA {
        empty: true,
        utf8: true,
        always_anchored: true,
    };

    let _flags = Flags::from_nfa(&nfa);
}

#[test]
fn test_from_nfa_all_flags_false() {
    struct MockNFA {
        empty: bool,
        utf8: bool,
        always_anchored: bool,
    }

    impl MockNFA {
        fn has_empty(&self) -> bool {
            self.empty
        }
        
        fn is_utf8(&self) -> bool {
            self.utf8
        }
        
        fn is_always_start_anchored(&self) -> bool {
            self.always_anchored
        }
    }

    let nfa = MockNFA {
        empty: false,
        utf8: false,
        always_anchored: false,
    };

    let _flags = Flags::from_nfa(&nfa);
}

#[test]
fn test_from_nfa_some_flags_true() {
    struct MockNFA {
        empty: bool,
        utf8: bool,
        always_anchored: bool,
    }

    impl MockNFA {
        fn has_empty(&self) -> bool {
            self.empty
        }
        
        fn is_utf8(&self) -> bool {
            self.utf8
        }
        
        fn is_always_start_anchored(&self) -> bool {
            self.always_anchored
        }
    }

    let nfa = MockNFA {
        empty: true,
        utf8: false,
        always_anchored: true,
    };

    let _flags = Flags::from_nfa(&nfa);
}

#[test]
fn test_from_nfa_other_combination() {
    struct MockNFA {
        empty: bool,
        utf8: bool,
        always_anchored: bool,
    }

    impl MockNFA {
        fn has_empty(&self) -> bool {
            self.empty
        }
        
        fn is_utf8(&self) -> bool {
            self.utf8
        }
        
        fn is_always_start_anchored(&self) -> bool {
            self.always_anchored
        }
    }

    let nfa = MockNFA {
        empty: false,
        utf8: true,
        always_anchored: false,
    };

    let _flags = Flags::from_nfa(&nfa);
}

