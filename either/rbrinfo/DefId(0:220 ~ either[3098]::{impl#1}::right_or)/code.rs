pub fn right_or(self, other: R) -> R {
        match self {
            Either::Left(_) => other,
            Either::Right(r) => r,
        }
    }