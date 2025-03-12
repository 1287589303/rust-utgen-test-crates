// Answer 0

#[test]
fn test_visit_pre_literal_single_control_byte() {
    use std::fmt::Write;
    
    struct TestWriter {
        output: String,
    }
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let bytes: &[u8] = &[0x00]; // single ASCII control byte
    let hir = Hir {
        kind: HirKind::Literal(hir::Literal(bytes)),
        props: Properties::default(),
    };
    
    let mut writer = TestWriter { output: String::new() };
    let mut visitor = Writer { wtr: &mut writer };
    
    let _result = visitor.visit_pre(&hir);
}

