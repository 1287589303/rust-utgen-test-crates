fn split_ascii_fast_path_prefix(label: &[u8]) -> (&[u8], &[u8]) {
    if let Some(pos) = label.iter().position(|b| !b.is_ascii()) {
        if pos == 0 {
            // First is non-ASCII
            (&[], label)
        } else {
            // Leave one ASCII character in the suffix
            // in case it's a letter that a combining
            // character combines with.
            let (head, tail) = label.split_at(pos - 1);
            (head, tail)
        }
    } else {
        // All ASCII
        (label, &[])
    }
}