fn source(&self) -> Option<&(dyn Error + 'static)> {
        for_both!(self, inner => inner.source())
    }