fn validate_ident(string: &str) {
    if string.is_empty() {
        panic!("Ident is not allowed to be empty; use Option<Ident>");
    }

    if string.bytes().all(|digit| b'0' <= digit && digit <= b'9') {
        panic!("Ident cannot be a number; use Literal instead");
    }

    fn ident_ok(string: &str) -> bool {
        let mut chars = string.chars();
        let first = chars.next().unwrap();
        if !is_ident_start(first) {
            return false;
        }
        for ch in chars {
            if !is_ident_continue(ch) {
                return false;
            }
        }
        true
    }

    if !ident_ok(string) {
        panic!("{:?} is not a valid Ident", string);
    }
}