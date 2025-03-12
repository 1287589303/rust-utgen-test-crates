pub fn c_string(string: &CStr) -> Literal {
        Literal::_new(imp::Literal::c_string(string))
    }