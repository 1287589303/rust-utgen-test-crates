pub(crate) fn write_label_len(label: &str) -> usize {
    if label.len() > 255 {
        panic!("label must not be longer than 255 bytes");
    }
    if label.as_bytes().iter().position(|&b| b == 0).is_some() {
        panic!("label must not contain NUL bytes");
    }
    let label_len = label.len() + 1; // +1 for the NUL terminator
    label_len + padding_len(label_len)
}