pub fn right_or_default(self) -> R
    where
        R: Default,
    {
        match self {
            Either::Left(_) => R::default(),
            Either::Right(r) => r,
        }
    }