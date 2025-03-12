// Answer 0

#[test]
fn test_serialize_newtype_variant_error_on_end_object_key() {
    struct TestFormatter;

    impl TestFormatter {
        fn begin_object(&mut self) -> Result<()> { Ok(()) }
        fn begin_object_key(&mut self, _: bool) -> Result<()> { Err(Error) }
        fn end_object_key(&mut self) -> Result<()> { Ok(()) }
        fn begin_object_value(&mut self) -> Result<()> { Ok(()) }
        fn end_object_value(&mut self) -> Result<()> { Ok(()) }
    }
    
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> { Ok(0) }
        fn write_all(&mut self, _: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct TestSerializer<W, F> {
        writer: W,
        formatter: F,
    }

    impl<W: io::Write, F: TestFormatter> ser::Serializer for TestSerializer<W, F> {
        fn serialize_str(self, _: &str) -> Result<()> { Ok(()) }
        fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            tri!(self.formatter.begin_object(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.begin_object_key(&mut self.writer, true).map_err(Error::io));
            tri!(self.serialize_str(variant));
            tri!(self.formatter.end_object_key(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.begin_object_value(&mut self.writer).map_err(Error::io));
            tri!(value.serialize(&mut self));
            tri!(self.formatter.end_object_value(&mut self.writer).map_err(Error::io));
            self.formatter.end_object(&mut self.writer).map_err(Error::io)
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = TestSerializer { writer, formatter };

    let result: Result<()> = serializer.serialize_newtype_variant("test_name", 0, "variant", &42);
}

#[test]
fn test_serialize_newtype_variant_error_on_serialize_str() {
    struct TestFormatter;

    impl TestFormatter {
        fn begin_object(&mut self) -> Result<()> { Ok(()) }
        fn begin_object_key(&mut self, _: bool) -> Result<()> { Ok(()) }
        fn end_object_key(&mut self) -> Result<()> { Ok(()) }
        fn begin_object_value(&mut self) -> Result<()> { Ok(()) }
        fn end_object_value(&mut self) -> Result<()> { Ok(()) }
    }
    
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> { Ok(0) }
        fn write_all(&mut self, _: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct TestSerializer<W, F> {
        writer: W,
        formatter: F,
    }

    impl<W: io::Write, F: TestFormatter> ser::Serializer for TestSerializer<W, F> {
        fn serialize_str(self, _: &str) -> Result<()> { Err(Error) }
        fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            tri!(self.formatter.begin_object(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.begin_object_key(&mut self.writer, true).map_err(Error::io));
            tri!(self.serialize_str(variant));
            tri!(self.formatter.end_object_key(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.begin_object_value(&mut self.writer).map_err(Error::io));
            tri!(value.serialize(&mut self));
            tri!(self.formatter.end_object_value(&mut self.writer).map_err(Error::io));
            self.formatter.end_object(&mut self.writer).map_err(Error::io)
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = TestSerializer { writer, formatter };

    let result: Result<()> = serializer.serialize_newtype_variant("test_name", 0, "variant", &42);
}

#[test]
fn test_serialize_newtype_variant_error_on_begin_object() {
    struct TestFormatter;

    impl TestFormatter {
        fn begin_object(&mut self) -> Result<()> { Err(Error) }
        fn begin_object_key(&mut self, _: bool) -> Result<()> { Ok(()) }
        fn end_object_key(&mut self) -> Result<()> { Ok(()) }
        fn begin_object_value(&mut self) -> Result<()> { Ok(()) }
        fn end_object_value(&mut self) -> Result<()> { Ok(()) }
    }
    
    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> { Ok(0) }
        fn write_all(&mut self, _: &[u8]) -> Result<()> { Ok(()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct TestSerializer<W, F> {
        writer: W,
        formatter: F,
    }

    impl<W: io::Write, F: TestFormatter> ser::Serializer for TestSerializer<W, F> {
        fn serialize_str(self, _: &str) -> Result<()> { Ok(()) }
        fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            tri!(self.formatter.begin_object(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.begin_object_key(&mut self.writer, true).map_err(Error::io));
            tri!(self.serialize_str(variant));
            tri!(self.formatter.end_object_key(&mut self.writer).map_err(Error::io));
            tri!(self.formatter.begin_object_value(&mut self.writer).map_err(Error::io));
            tri!(value.serialize(&mut self));
            tri!(self.formatter.end_object_value(&mut self.writer).map_err(Error::io));
            self.formatter.end_object(&mut self.writer).map_err(Error::io)
        }
    }

    let writer = TestWriter;
    let formatter = TestFormatter;
    let serializer = TestSerializer { writer, formatter };

    let result: Result<()> = serializer.serialize_newtype_variant("test_name", 0, "variant", &42);
}

