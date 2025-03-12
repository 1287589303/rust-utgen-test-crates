// Answer 0

#[test]
fn test_write_fmt_with_right_variant() {
    use std::fmt::Write;

    struct MockWriter(String);

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.0.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            write!(self, "{}", args)
        }
    }

    let mut writer = MockWriter(String::new());
    let mut either = Either::Right(writer);
    let args = format_args!("Hello, {}!", "world");
    
    let _ = either.write_fmt(args);
}

#[test]
fn test_write_fmt_with_empty_right_variant() {
    use std::fmt::Write;

    struct MockWriter(String);

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.0.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            write!(self, "{}", args)
        }
    }

    let mut writer = MockWriter(String::new());
    let mut either = Either::Right(writer);
    let args = format_args!("");
    
    let _ = either.write_fmt(args);
}

#[test]
fn test_write_fmt_with_special_chars_right_variant() {
    use std::fmt::Write;

    struct MockWriter(String);

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.0.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            write!(self, "{}", args)
        }
    }

    let mut writer = MockWriter(String::new());
    let mut either = Either::Right(writer);
    let args = format_args!("Special characters: !@#$%^&*()");
    
    let _ = either.write_fmt(args);
}

#[test]
fn test_write_fmt_with_multiple_arguments_right_variant() {
    use std::fmt::Write;

    struct MockWriter(String);

    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            self.0.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            write!(self, "{}", args)
        }
    }

    let mut writer = MockWriter(String::new());
    let mut either = Either::Right(writer);
    let args = format_args!("Multiple arguments: {}, {}", 42, "test");
    
    let _ = either.write_fmt(args);
}

