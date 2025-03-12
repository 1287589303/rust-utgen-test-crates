fn valid(src: &str) -> bool {
        TokenStream::from_str_checked(src).is_ok()
    }