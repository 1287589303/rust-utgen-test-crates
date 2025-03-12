// Answer 0

#[test]
fn test_visit_pre_with_capture_error() {
    struct WriterMock {
        output: String,
    }

    impl fmt::Write for WriterMock {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.output.len() > 0 {
                return Err(fmt::Error);
            }
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = WriterMock { output: String::new() };
    let capture = hir::Capture { name: Some("test_name".to_string()), ..Default::default() };
    let hir = Hir { kind: HirKind::Capture(capture), props: Default::default() };
    
    let result = writer.visit_pre(&hir);
    let _ = writer.finish();

    // Triggering the write! error
    let name = capture.name.as_ref().unwrap();
    let _ = write!(writer, "?P<{}>", name);
}

