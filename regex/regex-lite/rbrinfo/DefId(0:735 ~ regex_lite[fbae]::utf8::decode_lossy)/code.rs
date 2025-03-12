pub(crate) fn decode_lossy<B: AsRef<[u8]>>(slice: B) -> (char, usize) {
    match decode(slice) {
        (Some(ch), size) => (ch, size),
        (None, size) => ('\u{FFFD}', size),
    }
}