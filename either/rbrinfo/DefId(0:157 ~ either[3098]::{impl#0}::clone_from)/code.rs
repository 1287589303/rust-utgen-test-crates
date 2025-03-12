fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Left(dest), Left(source)) => dest.clone_from(source),
            (Right(dest), Right(source)) => dest.clone_from(source),
            (dest, source) => *dest = source.clone(),
        }
    }