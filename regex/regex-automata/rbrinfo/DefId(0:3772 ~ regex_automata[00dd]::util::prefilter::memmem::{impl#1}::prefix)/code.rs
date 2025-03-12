fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {
        #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))]
        {
            unreachable!()
        }
        #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
        {
            let needle = self.finder.needle();
            if haystack[span].starts_with(needle) {
                Some(Span { end: span.start + needle.len(), ..span })
            } else {
                None
            }
        }
    }