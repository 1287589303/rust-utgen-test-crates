fn ends_in_a_number(input: &str) -> bool {
    let mut parts = input.rsplit('.');
    let last = parts.next().unwrap();
    let last = if last.is_empty() {
        if let Some(last) = parts.next() {
            last
        } else {
            return false;
        }
    } else {
        last
    };
    if !last.is_empty() && last.as_bytes().iter().all(|c| c.is_ascii_digit()) {
        return true;
    }

    parse_ipv4number(last).is_ok()
}