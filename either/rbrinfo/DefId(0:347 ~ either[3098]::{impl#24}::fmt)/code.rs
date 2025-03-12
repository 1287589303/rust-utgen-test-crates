fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for_both!(self, inner => inner.fmt(f))
    }