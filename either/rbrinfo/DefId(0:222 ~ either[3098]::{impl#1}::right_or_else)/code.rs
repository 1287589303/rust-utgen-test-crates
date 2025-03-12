pub fn right_or_else<F>(self, f: F) -> R
    where
        F: FnOnce(L) -> R,
    {
        match self {
            Either::Left(l) => f(l),
            Either::Right(r) => r,
        }
    }