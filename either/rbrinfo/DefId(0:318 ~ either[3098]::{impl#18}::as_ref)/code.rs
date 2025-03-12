fn as_ref(&self) -> &[Target] {
        for_both!(self, inner => inner.as_ref())
    }