// Answer 0

#[test]
fn test_percent_encode_fmt_success() {
    struct MockFormatter {
        write_result: Result<(), fmt::Error>,
        written: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.written.push_str(s);
            self.write_result.clone()
        }
    }

    let bytes: &[u8] = b"valid\xE2\x80\xA8"; // includes valid and invalid ASCII
    let ascii_set = &percent_encoding::NON_ALPHANUMERIC; // or other suitable ASCII set
    let encode = percent_encoding::PercentEncode { bytes, ascii_set };
    let mut formatter = MockFormatter {
        write_result: Ok(()),
        written: String::new(),
    };

    let _ = encode.fmt(&mut formatter);
}

#[test]
fn test_percent_encode_fmt_failure() {
    struct MockFormatter {
        write_result: Result<(), fmt::Error>,
        written: String,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            self.write_result.clone()
        }
    }

    let bytes: &[u8] = b"invalid\xFF"; // includes invalid ASCII byte
    let ascii_set = &percent_encoding::NON_ALPHANUMERIC;
    let encode = percent_encoding::PercentEncode { bytes, ascii_set };
    let mut formatter = MockFormatter {
        write_result: Err(fmt::Error),
        written: String::new(),
    };

    let _ = encode.fmt(&mut formatter);
}

