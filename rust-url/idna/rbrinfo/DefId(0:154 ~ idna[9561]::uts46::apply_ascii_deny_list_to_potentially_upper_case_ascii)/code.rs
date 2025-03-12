fn apply_ascii_deny_list_to_potentially_upper_case_ascii(b: u8, deny_list: u128) -> char {
    if (deny_list & (1u128 << b)) == 0 {
        return char::from(b);
    }
    if in_inclusive_range8(b, b'A', b'Z') {
        return char::from(b + 0x20);
    }
    '\u{FFFD}'
}