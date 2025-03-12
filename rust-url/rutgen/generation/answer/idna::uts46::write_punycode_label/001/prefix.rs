// Answer 0

#[test]
#[should_panic]
fn test_write_punycode_label_sink_error() {
    struct FailingSink;

    impl core::fmt::Write for FailingSink {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            Err(core::fmt::Error)
        }
    }

    let label: &[char] = &['a', 'b', 'c'];
    let mut sink = FailingSink;

    let _ = write_punycode_label(label, &mut sink);
}

#[test]
fn test_write_punycode_label_empty_label() {
    struct Buffer {
        content: String,
    }

    impl core::fmt::Write for Buffer {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.content.push_str(s);
            Ok(())
        }
    }

    let label: &[char] = &[];
    let mut buffer = Buffer {
        content: String::new(),
    };

    let _ = write_punycode_label(label, &mut buffer);
}

#[test]
fn test_write_punycode_label_max_length_label() {
    struct Buffer {
        content: String,
    }

    impl core::fmt::Write for Buffer {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.content.push_str(s);
            Ok(())
        }
    }

    let label: Vec<char> = vec!['a'; 1000]; // 1000 characters
    let mut buffer = Buffer {
        content: String::new(),
    };

    let _ = write_punycode_label(&label, &mut buffer);
}

#[test]
fn test_write_punycode_label_boundary_case() {
    struct Buffer {
        content: String,
    }

    impl core::fmt::Write for Buffer {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.content.push_str(s);
            Ok(())
        }
    }

    let label: Vec<char> = vec!['a'; 999]; // 999 characters
    let mut buffer = Buffer {
        content: String::new(),
    };

    let _ = write_punycode_label(&label, &mut buffer);
}

#[test]
fn test_write_punycode_label_non_ascii_chars() {
    struct Buffer {
        content: String,
    }

    impl core::fmt::Write for Buffer {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            self.content.push_str(s);
            Ok(())
        }
    }

    let label: &[char] = &['з', 'а', 'р', 'я']; // Non-ASCII characters
    let mut buffer = Buffer {
        content: String::new(),
    };

    let _ = write_punycode_label(label, &mut buffer);
}

