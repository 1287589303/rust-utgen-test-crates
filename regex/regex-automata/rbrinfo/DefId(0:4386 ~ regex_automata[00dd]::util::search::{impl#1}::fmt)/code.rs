fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use crate::util::escape::DebugHaystack;

        f.debug_struct("Input")
            .field("haystack", &DebugHaystack(self.haystack()))
            .field("span", &self.span)
            .field("anchored", &self.anchored)
            .field("earliest", &self.earliest)
            .finish()
    }