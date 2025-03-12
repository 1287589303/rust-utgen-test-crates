fn into_either_with<F>(self, into_left: F) -> Either<Self, Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        let into_left = into_left(&self);
        self.into_either(into_left)
    }