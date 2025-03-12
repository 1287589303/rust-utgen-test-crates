// Answer 0

#[test]
fn test_fmt_class_set_binary_op_kind_intersection() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::ClassSetBinaryOpKind::Intersection;

    writer.fmt_class_set_binary_op_kind(&ast).unwrap();
}

#[test]
fn test_fmt_class_set_binary_op_kind_difference() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::ClassSetBinaryOpKind::Difference;

    writer.fmt_class_set_binary_op_kind(&ast).unwrap();
}

#[test]
fn test_fmt_class_set_binary_op_kind_symmetric_difference() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let ast = ast::ClassSetBinaryOpKind::SymmetricDifference;

    writer.fmt_class_set_binary_op_kind(&ast).unwrap();
}

