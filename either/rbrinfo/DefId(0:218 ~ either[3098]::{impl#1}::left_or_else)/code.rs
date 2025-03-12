pub fn left_or_else<F>(self, f: F) -> L
    where
        F: FnOnce(R) -> L,
    {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => f(r),
        }
    }