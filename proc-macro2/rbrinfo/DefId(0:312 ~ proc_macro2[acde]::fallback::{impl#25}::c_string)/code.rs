pub(crate) fn c_string(string: &CStr) -> Literal {
        let mut repr = "c\"".to_string();
        let mut bytes = string.to_bytes();
        while !bytes.is_empty() {
            let (valid, invalid) = match str::from_utf8(bytes) {
                Ok(all_valid) => {
                    bytes = b"";
                    (all_valid, bytes)
                }
                Err(utf8_error) => {
                    let (valid, rest) = bytes.split_at(utf8_error.valid_up_to());
                    let valid = str::from_utf8(valid).unwrap();
                    let invalid = utf8_error
                        .error_len()
                        .map_or(rest, |error_len| &rest[..error_len]);
                    bytes = &bytes[valid.len() + invalid.len()..];
                    (valid, invalid)
                }
            };
            escape_utf8(valid, &mut repr);
            for &byte in invalid {
                let _ = write!(repr, r"\x{:02X}", byte);
            }
        }
        repr.push('"');
        Literal::_new(repr)
    }