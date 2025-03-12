fn eq(&self, other: &Ident) -> bool {
        self.sym == other.sym && self.raw == other.raw
    }