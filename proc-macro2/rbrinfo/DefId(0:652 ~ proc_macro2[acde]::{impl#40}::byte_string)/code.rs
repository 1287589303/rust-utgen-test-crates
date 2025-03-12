pub fn byte_string(bytes: &[u8]) -> Literal {
        Literal::_new(imp::Literal::byte_string(bytes))
    }