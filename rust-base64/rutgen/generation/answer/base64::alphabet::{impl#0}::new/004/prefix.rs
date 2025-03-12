// Answer 0

#[test]
fn test_new_with_duplicate_bytes() {
    let alphabet = "ABCDEF" // 6 unique characters
        .chars()
        .chain("ABCDEFAABBCCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
        .collect::<String>();
    let result = Alphabet::new(&alphabet);
}

#[test]
fn test_new_with_leading_space_duplicate() {
    let alphabet = " A" // leading space and two A's
        .chars()
        .chain("BCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
        .collect::<String>();
    let result = Alphabet::new(&alphabet);
}

#[test]
fn test_new_with_trailing_space_duplicate() {
    let alphabet = "ABCDEF" // 6 unique and 2 trailing F's
        .chars()
        .chain("GHIJKLAVF") // A is duplicated
        .chain("MNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
        .collect::<String>();
    let result = Alphabet::new(&alphabet);
}

