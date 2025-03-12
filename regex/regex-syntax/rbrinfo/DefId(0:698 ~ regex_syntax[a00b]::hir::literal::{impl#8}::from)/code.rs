fn from(byte: u8) -> Literal {
        Literal::exact(vec![byte])
    }