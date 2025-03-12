// Answer 0

#[test]
fn test_visit_pre_literal_single_non_utf8_byte() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.extend_from_slice(s.as_bytes());
            Ok(())
        }
    }

    let byte: u8 = 0b11000000; // Non-UTF-8 byte
    let hir = Hir {
        kind: HirKind::Literal(hir::Literal(&[byte])),
        props: Properties::default(),
    };

    let mut writer = TestWriter { output: Vec::new() };
    let mut visitor = Writer { wtr: &mut writer };

    let _ = visitor.visit_pre(&hir);
}

