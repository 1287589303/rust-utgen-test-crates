pub fn escape(pattern: &str) -> String {
    let mut buf = String::new();
    buf.reserve(pattern.len());
    for ch in pattern.chars() {
        if is_meta_character(ch) {
            buf.push('\\');
        }
        buf.push(ch);
    }
    buf
}