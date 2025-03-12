pub(crate) fn from_str_unchecked(src: &str) -> Self {
        Self::from_str_checked(src).unwrap()
    }