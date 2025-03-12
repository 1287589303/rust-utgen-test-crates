// Answer 0

#[test]
fn test_fmt_class_set_binary_op_kind_intersection() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let ast = ast::ClassSetBinaryOpKind::Intersection;

    let _result = writer.fmt_class_set_binary_op_kind(&ast);
}

#[test]
fn test_fmt_class_set_binary_op_kind_difference() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let ast = ast::ClassSetBinaryOpKind::Difference;

    let _result = writer.fmt_class_set_binary_op_kind(&ast);
}

#[test]
fn test_fmt_class_set_binary_op_kind_symmetric_difference() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let ast = ast::ClassSetBinaryOpKind::SymmetricDifference;

    let _result = writer.fmt_class_set_binary_op_kind(&ast);
}

