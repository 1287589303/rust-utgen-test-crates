fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {
        #[cfg(not(feature = "perf-literal-multisubstring"))]
        {
            unreachable!()
        }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            use aho_corasick::automaton::Automaton;
            let input = aho_corasick::Input::new(haystack)
                .anchored(aho_corasick::Anchored::Yes)
                .span(span.start..span.end);
            self.anchored_ac
                .try_find(&input)
                // OK because we build the DFA with anchored support.
                .expect("aho-corasick DFA should never fail")
                .map(|m| Span { start: m.start(), end: m.end() })
        }
    }