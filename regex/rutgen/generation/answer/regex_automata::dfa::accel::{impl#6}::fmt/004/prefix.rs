// Answer 0

#[test]
fn test_fmt_ok_none_needles_error() {
    struct MockFormatter;

    impl core::fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let mut accel = Accel { bytes: [0; ACCEL_CAP] }; // empty bytes
    let mut formatter = MockFormatter;

    let _ = accel.fmt(&mut formatter);
}

#[test]
fn test_fmt_empty_needles_error() {
    struct MockFormatter;

    impl core::fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            Err(core::fmt::Error)
        }
    }

    let mut accel = Accel { bytes: [0, 0, 0, 0, 0, 0, 0, 0] }; // first byte as 0, rest does not matter
    let mut formatter = MockFormatter;

    let _ = accel.fmt(&mut formatter);
}

