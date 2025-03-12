fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {
        #[cfg(not(feature = "perf-literal-substring"))]
        {
            unreachable!()
        }
        #[cfg(feature = "perf-literal-substring")]
        {
            memchr::memchr3(self.0, self.1, self.2, &haystack[span]).map(|i| {
                let start = span.start + i;
                let end = start + 1;
                Span { start, end }
            })
        }
    }