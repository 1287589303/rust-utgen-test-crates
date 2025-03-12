pub fn maximum_len(&self) -> Option<usize> {
        match *self {
            Class::Unicode(ref x) => x.maximum_len(),
            Class::Bytes(ref x) => x.maximum_len(),
        }
    }