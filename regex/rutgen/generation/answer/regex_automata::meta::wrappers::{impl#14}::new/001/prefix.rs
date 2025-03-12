// Answer 0

#[test]
fn test_reverse_hybrid_new_with_hybrid_enabled() {
    let regex_info = {
        let config = {
            let mut builder = regex_automata::meta::regex::ConfigBuilder::default();
            builder.set_hybrid(true);
            builder.build()
        };
        RegexInfo(Arc::new(RegexInfoI { config }))
    };

    let nfa = {
        let inner = Arc::new(Inner::new());
        NFA(inner)
    };

    let reverse_hybrid = ReverseHybrid::new(&regex_info, &nfa);
}

#[test]
fn test_reverse_hybrid_new_with_hybrid_disabled() {
    let regex_info = {
        let config = {
            let mut builder = regex_automata::meta::regex::ConfigBuilder::default();
            builder.set_hybrid(false);
            builder.build()
        };
        RegexInfo(Arc::new(RegexInfoI { config }))
    };

    let nfa = {
        let inner = Arc::new(Inner::new());
        NFA(inner)
    };

    let reverse_hybrid = ReverseHybrid::new(&regex_info, &nfa);
}

