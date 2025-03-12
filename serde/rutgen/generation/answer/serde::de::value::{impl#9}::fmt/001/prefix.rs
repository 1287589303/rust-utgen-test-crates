// Answer 0

#[test]
fn test_unit_deserializer_formatting() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut formatter = TestFormatter;
    let deserializer = UnitDeserializer { marker: PhantomData::<()>::default() };
    deserializer.fmt(&mut formatter);
}

#[test]
fn test_unit_deserializer_formatting_empty() {
    struct EmptyFormatter;

    impl fmt::Write for EmptyFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut formatter = EmptyFormatter;
    let deserializer = UnitDeserializer { marker: PhantomData::<()>::default() };
    deserializer.fmt(&mut formatter);
}

