pub fn left_or_default(self) -> L
    where
        L: Default,
    {
        match self {
            Either::Left(l) => l,
            Either::Right(_) => L::default(),
        }
    }