fn write_str(&mut self, s: &str) -> fmt::Result {
        for_both!(self, inner => inner.write_str(s))
    }