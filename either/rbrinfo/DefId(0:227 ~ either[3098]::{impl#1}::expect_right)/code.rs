pub fn expect_right(self, msg: &str) -> R
    where
        L: core::fmt::Debug,
    {
        match self {
            Either::Right(r) => r,
            Either::Left(l) => panic!("{}: {:?}", msg, l),
        }
    }