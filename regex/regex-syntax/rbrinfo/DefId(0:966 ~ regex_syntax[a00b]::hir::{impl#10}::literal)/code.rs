pub fn literal(&self) -> Option<Vec<u8>> {
        match *self {
            Class::Unicode(ref x) => x.literal(),
            Class::Bytes(ref x) => x.literal(),
        }
    }