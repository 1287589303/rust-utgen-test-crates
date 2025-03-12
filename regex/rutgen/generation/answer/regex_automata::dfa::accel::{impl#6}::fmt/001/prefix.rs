// Answer 0

#[test]
fn test_fmt_with_err_formatter() {
    struct ErrFormatter;

    impl core::fmt::Write for ErrFormatter {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            Err(core::fmt::Error)
        }
    }

    let accel = Accel {
        bytes: [0; ACCEL_CAP],
    };

    let mut formatter = ErrFormatter;
    let _ = accel.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_no_capacity_formatter() {
    struct NoCapacityFormatter;

    impl core::fmt::Write for NoCapacityFormatter {
        fn write_str(&mut self, _: &str) -> core::fmt::Result {
            // Simulate a situation where the buffer effectively has no capacity
            Err(core::fmt::Error)
        }
    }

    let accel = Accel {
        bytes: [0; ACCEL_CAP],
    };

    let mut formatter = NoCapacityFormatter;
    let _ = accel.fmt(&mut formatter);
}

