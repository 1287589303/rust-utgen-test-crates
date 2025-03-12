fn valid(src: &str) -> bool {
        Literal::from_str_checked(src).is_ok()
    }