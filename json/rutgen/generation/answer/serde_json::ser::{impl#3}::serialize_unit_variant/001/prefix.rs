// Answer 0

#[test]
fn test_serialize_unit_variant_valid_case() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let serializer = &mut Serializer {
        writer,
        formatter: CompactFormatter,
    };

    let _result = serializer.serialize_unit_variant("test_name", 0, "test_variant");
}

#[test]
fn test_serialize_unit_variant_non_empty_name() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let serializer = &mut Serializer {
        writer,
        formatter: CompactFormatter,
    };

    let _result = serializer.serialize_unit_variant("valid_name", 1, "another_variant");
}

#[test]
fn test_serialize_unit_variant_with_different_variant_index() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let serializer = &mut Serializer {
        writer,
        formatter: CompactFormatter,
    };

    let _result = serializer.serialize_unit_variant("test_name", 2, "yet_another_variant");
}

#[test]
fn test_serialize_unit_variant_edge_case() {
    struct TestWriter;
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter;
    let serializer = &mut Serializer {
        writer,
        formatter: CompactFormatter,
    };

    let _result = serializer.serialize_unit_variant("empty_variant_name", 0, "");
}

