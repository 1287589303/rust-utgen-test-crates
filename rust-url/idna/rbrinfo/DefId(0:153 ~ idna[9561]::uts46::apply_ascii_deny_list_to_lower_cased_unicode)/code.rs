fn apply_ascii_deny_list_to_lower_cased_unicode(c: char, deny_list: u128) -> char {
    if let Some(shifted) = 1u128.checked_shl(u32::from(c)) {
        if (deny_list & shifted) == 0 {
            c
        } else {
            '\u{FFFD}'
        }
    } else {
        c
    }
}