pub fn left_or(self, other: L) -> L {
        match self {
            Either::Left(l) => l,
            Either::Right(_) => other,
        }
    }