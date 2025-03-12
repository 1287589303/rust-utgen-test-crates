fn description(&self) -> &str {
        for_both!(self, inner => inner.description())
    }