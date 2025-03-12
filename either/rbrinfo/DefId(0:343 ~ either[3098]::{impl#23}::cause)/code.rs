fn cause(&self) -> Option<&dyn Error> {
        for_both!(self, inner => inner.cause())
    }