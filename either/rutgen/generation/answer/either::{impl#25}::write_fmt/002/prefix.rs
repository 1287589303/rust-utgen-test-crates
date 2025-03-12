// Answer 0

#[test]
fn test_write_fmt_left_with_string() {
    struct WriteString {
        content: String,
    }

    impl fmt::Write for WriteString {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.content.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.content.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            write!(self, "{}", args)
        }
    }

    let mut left_value = Left(WriteString { content: String::new() });
    let args = format_args!("Hello, {}!", "world");
    let _ = write_fmt(&mut left_value, args);
}

#[test]
fn test_write_fmt_right_with_string() {
    struct WriteString {
        content: String,
    }
    
    impl fmt::Write for WriteString {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.content.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.content.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            write!(self, "{}", args)
        }
    }

    let mut right_value = Right(WriteString { content: String::new() });
    let args = format_args!("Goodbye, {}!", "world");
    let _ = write_fmt(&mut right_value, args);
}

#[test]
fn test_write_fmt_left_with_empty_string() {
    struct WriteString {
        content: String,
    }

    impl fmt::Write for WriteString {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.content.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.content.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            write!(self, "{}", args)
        }
    }

    let mut left_value = Left(WriteString { content: String::new() });
    let args = format_args!("{}");
    let _ = write_fmt(&mut left_value, args);
}

#[test]
fn test_write_fmt_right_with_large_input() {
    struct WriteString {
        content: String,
    }

    impl fmt::Write for WriteString {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.content.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.content.push(c);
            Ok(())
        }

        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            write!(self, "{}", args)
        }
    }

    let mut right_value = Right(WriteString { content: String::new() });
    let large_str = "x".repeat(10_000);
    let args = format_args!("{}", large_str);
    let _ = write_fmt(&mut right_value, args);
}

