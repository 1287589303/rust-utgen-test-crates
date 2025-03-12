pub fn inexact<B: Into<Vec<u8>>>(bytes: B) -> Literal {
        Literal { bytes: bytes.into(), exact: false }
    }