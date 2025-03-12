pub fn exact<B: Into<Vec<u8>>>(bytes: B) -> Literal {
        Literal { bytes: bytes.into(), exact: true }
    }