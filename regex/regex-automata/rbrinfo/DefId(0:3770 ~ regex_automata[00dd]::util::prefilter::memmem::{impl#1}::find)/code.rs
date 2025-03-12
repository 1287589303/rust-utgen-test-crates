fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {
        #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))]
        {
            unreachable!()
        }
        #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
        {
            self.finder.find(&haystack[span]).map(|i| {
                let start = span.start + i;
                let end = start + self.finder.needle().len();
                Span { start, end }
            })
        }
    }