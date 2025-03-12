pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        map_either!(self, inner => inner)
    }