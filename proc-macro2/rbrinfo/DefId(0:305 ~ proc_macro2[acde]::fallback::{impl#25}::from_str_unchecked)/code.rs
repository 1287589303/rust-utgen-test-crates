pub(crate) unsafe fn from_str_unchecked(repr: &str) -> Self {
        Literal::_new(repr.to_owned())
    }