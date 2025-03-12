pub(crate) fn byte_character(byte: u8) -> Literal {
        let mut repr = "b'".to_string();
        #[allow(clippy::match_overlapping_arm)]
        match byte {
            b'\0' => repr.push_str(r"\0"),
            b'\t' => repr.push_str(r"\t"),
            b'\n' => repr.push_str(r"\n"),
            b'\r' => repr.push_str(r"\r"),
            b'\'' => repr.push_str(r"\'"),
            b'\\' => repr.push_str(r"\\"),
            b'\x20'..=b'\x7E' => repr.push(byte as char),
            _ => {
                let _ = write!(repr, r"\x{:02X}", byte);
            }
        }
        repr.push('\'');
        Literal::_new(repr)
    }