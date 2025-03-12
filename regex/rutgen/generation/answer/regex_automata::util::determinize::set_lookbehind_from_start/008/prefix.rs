// Answer 0

#[test]
fn test_set_lookbehind_from_start_line_cr_rev_contains_anchor_crlf() {
    let mut builder = StateBuilderMatches(vec![]);
    let nfa = {
        struct DummyNFA;
        impl thompson::NFA for DummyNFA {
            fn is_reverse(&self) -> bool {
                true
            }
            fn look_matcher(&self) -> &LookMatcher {
                &LookMatcher { lineterm: DebugByte(0) }  // lineterm can be any value != b'\r'
            }
            fn look_set_any(&self) -> LookSet {
                let mut lookset = LookSet::empty();
                lookset.set_insert(Look::StartCRLF);
                lookset.set_insert(Look::StartLF);
                lookset  // contains_anchor_crlf() and contains_anchor_line() must be true
            }
        }
        DummyNFA
    };

    let start = Start::LineCR;

    set_lookbehind_from_start(&nfa, &start, &mut builder);
}

