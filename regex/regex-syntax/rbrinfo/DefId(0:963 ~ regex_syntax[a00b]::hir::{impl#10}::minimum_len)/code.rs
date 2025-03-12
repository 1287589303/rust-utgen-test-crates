pub fn minimum_len(&self) -> Option<usize> {
        match *self {
            Class::Unicode(ref x) => x.minimum_len(),
            Class::Bytes(ref x) => x.minimum_len(),
        }
    }