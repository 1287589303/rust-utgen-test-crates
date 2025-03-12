pub fn is_empty(&self) -> bool {
        match *self {
            Class::Unicode(ref x) => x.ranges().is_empty(),
            Class::Bytes(ref x) => x.ranges().is_empty(),
        }
    }