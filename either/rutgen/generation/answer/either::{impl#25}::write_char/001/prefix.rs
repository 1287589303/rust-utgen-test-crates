// Answer 0

#[test]
fn test_write_char_right_case() {
    struct MockWriter {
        buffer: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.buffer.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, _args: fmt::Arguments<'_>) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut either = Either::Right(writer);
    let valid_char = 'a';

    let _ = either.write_char(valid_char);
}

#[test]
fn test_write_char_boundary_case_min() {
    struct MockWriter {
        buffer: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.buffer.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, _args: fmt::Arguments<'_>) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut either = Either::Right(writer);
    let min_char = '\u{0000}'; // Unicode minimum

    let _ = either.write_char(min_char);
}

#[test]
fn test_write_char_boundary_case_max() {
    struct MockWriter {
        buffer: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.buffer.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, _args: fmt::Arguments<'_>) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut either = Either::Right(writer);
    let max_char = '\u{10FFFF}'; // Unicode maximum

    let _ = either.write_char(max_char);
}

#[test]
fn test_write_char_valid_extended() {
    struct MockWriter {
        buffer: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.buffer.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, _args: fmt::Arguments<'_>) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: String::new() };
    let mut either = Either::Right(writer);
    let valid_char_extended = '😊'; // Valid Unicode character

    let _ = either.write_char(valid_char_extended);
}

