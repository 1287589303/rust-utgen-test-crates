fn repr(&self) -> Repr<'_> {
        Repr(&*self.0)
    }