pub fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {
        #[cfg(not(feature = "alloc"))]
        {
            unreachable!()
        }
        #[cfg(feature = "alloc")]
        {
            self.pre.find(haystack, span)
        }
    }