fn repr(&self) -> Repr<'_> {
        Repr(self.0.as_slice())
    }