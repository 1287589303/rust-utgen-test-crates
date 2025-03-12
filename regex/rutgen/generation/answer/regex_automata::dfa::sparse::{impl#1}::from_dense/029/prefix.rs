// Answer 0

#[cfg(test)]
fn test_from_dense_valid() {
    struct MockDenseDFA;
    
    impl dense::DFA<&[u32]> {
        fn new() -> Self {
            MockDenseDFA
        }
        
        fn state_len(&self) -> usize {
            1
        }
        
        fn states(&self) -> impl Iterator<Item = State<'_>> {
            std::iter::once(State {
                id: StateID(0),
                sparse_transitions: || {
                    std::iter::once((Unit::u8(1), Unit::u8(2), 0))
                },
                is_match: false,
            })
        }

        fn to_index(&self, id: StateID) -> usize {
            id.0 as usize
        }

        fn is_match_state(&self, _id: StateID) -> bool {
            false
        }

        fn byte_classes(&self) -> ByteClasses {
            ByteClasses([0; 256])
        }

        fn accelerator(&self, _id: StateID) -> &[u8] {
            &[1, 2, 3]
        }

        fn match_pattern_len(&self, _id: StateID) -> usize {
            1
        }

        fn pattern_id_slice(&self, _id: StateID) -> &[PatternID] {
            &[PatternID(0)]
        }

        fn special(&self) -> &Special {
            &Special {
                max: StateID(1),
                quit_id: StateID(0),
                min_match: StateID(0),
                max_match: StateID(1),
                min_accel: StateID(0),
                max_accel: StateID(1),
                min_start: StateID(0),
                max_start: StateID(1),
            }
        }

        fn quitset(&self) -> &ByteSet {
            &ByteSet([false; 256])
        }

        fn flags(&self) -> &Flags {
            &Flags {
                has_empty: false,
                is_utf8: false,
                is_always_start_anchored: false,
            }
        }
    }
    
    let dense_dfa = MockDenseDFA::new();
    let result = from_dense(&dense_dfa);
}

#[cfg(test)]
#[should_panic]
fn test_from_dense_invalid_state_id() {
    struct InvalidMockDenseDFA;

    impl dense::DFA<&[u32]> {
        fn new() -> Self {
            InvalidMockDenseDFA
        }
        
        fn state_len(&self) -> usize {
            1
        }
        
        fn states(&self) -> impl Iterator<Item = State<'_>> {
            std::iter::once(State {
                id: StateID(0),
                sparse_transitions: || {
                    std::iter::once((Unit::u8(1), Unit::u8(2), 0))
                },
                is_match: true,
            })
        }

        fn to_index(&self, id: StateID) -> usize {
            id.0 as usize
        }

        fn is_match_state(&self, _id: StateID) -> bool {
            true
        }

        fn byte_classes(&self) -> ByteClasses {
            ByteClasses([0; 256])
        }

        fn accelerator(&self, _id: StateID) -> &[u8] {
            &[]
        }

        fn match_pattern_len(&self, _id: StateID) -> usize {
            256  // To trigger an error
        }

        fn pattern_id_slice(&self, _id: StateID) -> &[PatternID] {
            &[PatternID(0)]
        }

        fn special(&self) -> &Special {
            &Special {
                max: StateID(1),
                quit_id: StateID(0),
                min_match: StateID(0),
                max_match: StateID(1),
                min_accel: StateID(0),
                max_accel: StateID(1),
                min_start: StateID(0),
                max_start: StateID(1),
            }
        }

        fn quitset(&self) -> &ByteSet {
            &ByteSet([false; 256])
        }

        fn flags(&self) -> &Flags {
            &Flags {
                has_empty: false,
                is_utf8: false,
                is_always_start_anchored: false,
            }
        }
    }
    
    let dense_dfa = InvalidMockDenseDFA::new();
    let _result = from_dense(&dense_dfa);
}

