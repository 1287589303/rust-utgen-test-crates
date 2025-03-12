fn deref_mut(&mut self) -> &mut Self::Target {
        for_both!(self, inner => &mut *inner)
    }