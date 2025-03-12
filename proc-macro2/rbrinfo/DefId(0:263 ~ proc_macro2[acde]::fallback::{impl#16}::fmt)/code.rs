fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(span_locations)]
        return write!(f, "bytes({}..{})", self.lo, self.hi);

        #[cfg(not(span_locations))]
        write!(f, "Span")
    }