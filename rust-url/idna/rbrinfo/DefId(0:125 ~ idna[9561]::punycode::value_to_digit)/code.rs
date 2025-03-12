fn value_to_digit(value: u32) -> char {
    match value {
        0..=25 => (value as u8 + b'a') as char,       // a..z
        26..=35 => (value as u8 - 26 + b'0') as char, // 0..9
        _ => panic!(),
    }
}