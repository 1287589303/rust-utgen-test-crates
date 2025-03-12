pub fn expect_left(self, msg: &str) -> L
    where
        R: core::fmt::Debug,
    {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => panic!("{}: {:?}", msg, r),
        }
    }