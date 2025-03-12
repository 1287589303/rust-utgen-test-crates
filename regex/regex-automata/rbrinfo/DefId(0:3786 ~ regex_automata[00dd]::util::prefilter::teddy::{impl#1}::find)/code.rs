fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {
        #[cfg(not(feature = "perf-literal-multisubstring"))]
        {
            unreachable!()
        }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            let ac_span =
                aho_corasick::Span { start: span.start, end: span.end };
            self.searcher
                .find_in(haystack, ac_span)
                .map(|m| Span { start: m.start(), end: m.end() })
        }
    }