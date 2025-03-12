// Answer 0

#[test]
fn test_fmt_class_perl_word_negated() {
    struct StringWriter {
        output: String,
    }
    
    impl fmt::Write for StringWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = StringWriter { output: String::new() };
    
    let ast = ast::ClassPerl {
        span: Span::default(), 
        kind: ClassPerlKind::Word, 
        negated: true,
    };
    
    writer.fmt_class_perl(&ast).unwrap();
}

#[test]
fn test_fmt_class_perl_word_non_negated() {
    struct StringWriter {
        output: String,
    }
    
    impl fmt::Write for StringWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = StringWriter { output: String::new() };
    
    let ast = ast::ClassPerl {
        span: Span::default(), 
        kind: ClassPerlKind::Word, 
        negated: false,
    };
    
    writer.fmt_class_perl(&ast).unwrap();
}

