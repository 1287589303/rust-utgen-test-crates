// Answer 0

#[test]
fn test_visit_class_set_item_post_unicode_one_letter() {
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
    let unicode_class = ast::ClassUnicode {
        span: Span { start: 0, end: 1 },
        negated: false,
        kind: ClassUnicodeKind::OneLetter('a'),
    };

    let class_set_item = ast::ClassSetItem::Unicode(unicode_class);
    let mut visitor = Writer { wtr: writer };

    let _ = visitor.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_unicode_negated() {
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
    let unicode_class = ast::ClassUnicode {
        span: Span { start: 0, end: 1 },
        negated: true,
        kind: ClassUnicodeKind::OneLetter('b'),
    };

    let class_set_item = ast::ClassSetItem::Unicode(unicode_class);
    let mut visitor = Writer { wtr: writer };

    let _ = visitor.visit_class_set_item_post(&class_set_item);
}

