pub fn string(string: &str) -> Literal {
        Literal::_new(imp::Literal::string(string))
    }