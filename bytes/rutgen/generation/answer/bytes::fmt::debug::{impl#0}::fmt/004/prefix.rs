// Answer 0

#[test]
fn test_fmt_with_mixed_ascii_and_non_ascii_without_newline() {
    let data = b"Hello, world!\x01\x02\x03";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let _ = write!(&mut output, "{:?}", bytes_ref);
}

#[test]
fn test_fmt_with_special_characters_without_newline() {
    let data = b"Line 1\\nLine 2\\rLine 3\\tEnd\x04";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let _ = write!(&mut output, "{:?}", bytes_ref);
}

#[test]
fn test_fmt_with_no_special_characters_without_newline() {
    let data = b"SimpleText";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let _ = write!(&mut output, "{:?}", bytes_ref);
}

#[test]
fn test_fmt_with_non_printable_and_special_characters_without_newline() {
    let data = b"\x0F\x1AHello!\x80\xFF";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let _ = write!(&mut output, "{:?}", bytes_ref);
}

#[test]
fn test_fmt_with_edges_of_byte_range_without_newline() {
    let data = b"\x20\x21\x7E\x7F";
    let bytes_ref = BytesRef(data);
    let mut output = Vec::new();
    let _ = write!(&mut output, "{:?}", bytes_ref);
}

