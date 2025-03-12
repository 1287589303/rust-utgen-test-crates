// Answer 0

#[test]
fn test_size_hint_empty_bytes() {
    static ASCII_SET: AsciiSet = AsciiSet { mask: [0; ASCII_RANGE_LEN / BITS_PER_CHUNK] };
    let percent_encode = PercentEncode {
        bytes: &[],
        ascii_set: &ASCII_SET,
    };
    percent_encode.size_hint();
}

