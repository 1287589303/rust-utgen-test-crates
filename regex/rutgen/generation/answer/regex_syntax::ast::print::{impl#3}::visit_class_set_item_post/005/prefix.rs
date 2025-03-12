// Answer 0

#[test]
fn test_visit_class_set_item_post_ascii_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let ascii_class = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };
    let class_set_item = ast::ClassSetItem::Ascii(ascii_class);
    
    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_ascii_non_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let ascii_class = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };
    let class_set_item = ast::ClassSetItem::Ascii(ascii_class);
    
    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_class_set_item_post(&class_set_item);
}

