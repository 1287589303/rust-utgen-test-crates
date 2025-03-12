// Answer 0

#[test]
fn test_accel_fmt_empty() {
    let mut formatter = core::fmt::Formatter::new();
    let accel = Accel {
        bytes: [0; ACCEL_CAP], // bytes initialized to zero, representing an empty state
    };
    let _ = accel.fmt(&mut formatter);
}

#[test]
fn test_accel_fmt_empty_len() {
    let mut formatter = core::fmt::Formatter::new();
    let accel = Accel {
        bytes: [0; ACCEL_CAP], // bytes initialized to zero
    };
    let _ = accel.fmt(&mut formatter);
}

