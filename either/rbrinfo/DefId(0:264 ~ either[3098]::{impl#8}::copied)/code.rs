pub fn copied(self) -> Either<L, R>
    where
        L: Copy,
        R: Copy,
    {
        map_either!(self, inner => *inner)
    }