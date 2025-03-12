// Answer 0

#[test]
fn test_write_char_left() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            write!(self.output, "{}", args)
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut either = Either::Left(writer);
    let char_to_write = 'a';
    let _ = either.write_char(char_to_write);
}

#[test]
fn test_write_char_right() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            write!(self.output, "{}", args)
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut either = Either::Right(writer);
    let char_to_write = 'b';
    let _ = either.write_char(char_to_write);
}

