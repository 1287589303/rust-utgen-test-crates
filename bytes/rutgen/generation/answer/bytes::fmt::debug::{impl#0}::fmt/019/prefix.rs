// Answer 0

#[test]
fn test_debug_bytesref_with_newline() {
    let data = BytesRef(&[b'H', b'e', b'l', b'l', b'o', b'\n', b'W', b'o', b'r', b'l', b'd']);
    let mut buffer = vec![];
    let _ = write!(&mut buffer, "{:?}", data);
}

#[test]
fn test_debug_bytesref_with_carriage_return() {
    let data = BytesRef(&[b'T', b'e', b's', b't', b'\r', b'A', b'B', b'C']);
    let mut buffer = vec![];
    let _ = write!(&mut buffer, "{:?}", data);
}

#[test]
fn test_debug_bytesref_with_tab() {
    let data = BytesRef(&[b'1', b'\t', b'2', b'\t', b'3']);
    let mut buffer = vec![];
    let _ = write!(&mut buffer, "{:?}", data);
}

#[test]
fn test_debug_bytesref_with_backslash() {
    let data = BytesRef(&[b'T', b'e', b's', b't', b'\\', b'B', b'y', b't', b'e']);
    let mut buffer = vec![];
    let _ = write!(&mut buffer, "{:?}", data);
}

#[test]
fn test_debug_bytesref_with_quote() {
    let data = BytesRef(&[b'F', b'ø', b'c', b'\"', b'k']);
    let mut buffer = vec![];
    let _ = write!(&mut buffer, "{:?}", data);
}

#[test]
fn test_debug_bytesref_with_non_printable() {
    let data = BytesRef(&[b'\x1F', b'A', b'B', b'C', b'\x7F']);
    let mut buffer = vec![];
    let _ = write!(&mut buffer, "{:?}", data);
}

