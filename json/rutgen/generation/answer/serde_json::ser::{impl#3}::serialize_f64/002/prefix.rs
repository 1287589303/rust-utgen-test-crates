// Answer 0

#[test]
fn test_serialize_f64_nan() {
    struct TestFormatter;
    impl TestFormatter {
        fn write_null(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_f64(&mut self, writer: &mut dyn io::Write, _value: f64) -> Result<()> {
            Ok(())
        }
    }

    struct TestSerializer {
        writer: Vec<u8>,
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self {
                writer: Vec::new(),
                formatter: TestFormatter,
            }
        }
    }

    let mut serializer = TestSerializer::new();
    let value = f64::NAN;
    let _ = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_positive_infinity() {
    struct TestFormatter;
    impl TestFormatter {
        fn write_null(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_f64(&mut self, writer: &mut dyn io::Write, _value: f64) -> Result<()> {
            Ok(())
        }
    }

    struct TestSerializer {
        writer: Vec<u8>,
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self {
                writer: Vec::new(),
                formatter: TestFormatter,
            }
        }
    }

    let mut serializer = TestSerializer::new();
    let value = f64::INFINITY;
    let _ = serializer.serialize_f64(value);
}

#[test]
fn test_serialize_f64_negative_infinity() {
    struct TestFormatter;
    impl TestFormatter {
        fn write_null(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_f64(&mut self, writer: &mut dyn io::Write, _value: f64) -> Result<()> {
            Ok(())
        }
    }

    struct TestSerializer {
        writer: Vec<u8>,
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self {
                writer: Vec::new(),
                formatter: TestFormatter,
            }
        }
    }

    let mut serializer = TestSerializer::new();
    let value = f64::NEG_INFINITY;
    let _ = serializer.serialize_f64(value);
}

