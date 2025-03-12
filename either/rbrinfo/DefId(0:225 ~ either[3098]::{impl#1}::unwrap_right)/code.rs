pub fn unwrap_right(self) -> R
    where
        L: core::fmt::Debug,
    {
        match self {
            Either::Right(r) => r,
            Either::Left(l) => panic!("called `Either::unwrap_right()` on a `Left` value: {:?}", l),
        }
    }