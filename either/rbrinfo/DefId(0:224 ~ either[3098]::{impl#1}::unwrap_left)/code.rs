pub fn unwrap_left(self) -> L
    where
        R: core::fmt::Debug,
    {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => {
                panic!("called `Either::unwrap_left()` on a `Right` value: {:?}", r)
            }
        }
    }