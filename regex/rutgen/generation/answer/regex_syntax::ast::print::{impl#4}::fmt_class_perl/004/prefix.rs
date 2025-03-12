// Answer 0

#[test]
fn test_fmt_class_perl_space_negated() {
    struct WriterMock {
        output: String,
    }
    
    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };
    
    let ast = ast::ClassPerl {
        span: todo!(), // Replace with actual span value as needed
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };
    
    let _ = writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_space_non_negated() {
    struct WriterMock {
        output: String,
    }
    
    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };
    
    let ast = ast::ClassPerl {
        span: todo!(), // Replace with actual span value as needed
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };
    
    let _ = writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_digit_negated() {
    struct WriterMock {
        output: String,
    }
    
    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };
    
    let ast = ast::ClassPerl {
        span: todo!(), // Replace with actual span value as needed
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };
    
    let _ = writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_digit_non_negated() {
    struct WriterMock {
        output: String,
    }
    
    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };
    
    let ast = ast::ClassPerl {
        span: todo!(), // Replace with actual span value as needed
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    
    let _ = writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_word_negated() {
    struct WriterMock {
        output: String,
    }
    
    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };
    
    let ast = ast::ClassPerl {
        span: todo!(), // Replace with actual span value as needed
        kind: ast::ClassPerlKind::Word,
        negated: true,
    };
    
    let _ = writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_word_non_negated() {
    struct WriterMock {
        output: String,
    }
    
    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };
    
    let ast = ast::ClassPerl {
        span: todo!(), // Replace with actual span value as needed
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };
    
    let _ = writer.fmt_class_perl(&ast);
}

