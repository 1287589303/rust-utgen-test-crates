fn as_mut(&mut self) -> &mut Target {
        for_both!(self, inner => inner.as_mut())
    }