fn from(ch: char) -> Literal {
        use alloc::string::ToString;
        Literal::exact(ch.encode_utf8(&mut [0; 4]).to_string())
    }