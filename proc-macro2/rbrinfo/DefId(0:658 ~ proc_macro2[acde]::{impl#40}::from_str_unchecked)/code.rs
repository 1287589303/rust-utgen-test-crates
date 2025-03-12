pub unsafe fn from_str_unchecked(repr: &str) -> Self {
        Literal::_new(unsafe { imp::Literal::from_str_unchecked(repr) })
    }