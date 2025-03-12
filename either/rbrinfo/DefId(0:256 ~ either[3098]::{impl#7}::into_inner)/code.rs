pub fn into_inner(self) -> T {
        for_both!(self, inner => inner)
    }