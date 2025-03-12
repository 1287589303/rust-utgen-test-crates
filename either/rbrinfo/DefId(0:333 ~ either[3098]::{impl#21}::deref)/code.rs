fn deref(&self) -> &Self::Target {
        for_both!(self, inner => &**inner)
    }