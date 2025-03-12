pub(crate) fn byte_string(bytes: &[u8]) -> Literal {
        let mut repr = "b\"".to_string();
        let mut bytes = bytes.iter();
        while let Some(&b) = bytes.next() {
            #[allow(clippy::match_overlapping_arm)]
            match b {
                b'\0' => repr.push_str(match bytes.as_slice().first() {
                    // circumvent clippy::octal_escapes lint
                    Some(b'0'..=b'7') => r"\x00",
                    _ => r"\0",
                }),
                b'\t' => repr.push_str(r"\t"),
                b'\n' => repr.push_str(r"\n"),
                b'\r' => repr.push_str(r"\r"),
                b'"' => repr.push_str("\\\""),
                b'\\' => repr.push_str(r"\\"),
                b'\x20'..=b'\x7E' => repr.push(b as char),
                _ => {
                    let _ = write!(repr, r"\x{:02X}", b);
                }
            }
        }
        repr.push('"');
        Literal::_new(repr)
    }