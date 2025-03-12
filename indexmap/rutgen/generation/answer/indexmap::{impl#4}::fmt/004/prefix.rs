// Answer 0

#[test]
fn test_fmt_capacity_overflow() {
    let capacity_overflow_error = TryReserveError {
        kind: TryReserveErrorKind::CapacityOverflow,
    };
    let mut buffer = Vec::new();
    let mut formatter = core::fmt::Formatter::new(&mut buffer);
    let _ = capacity_overflow_error.fmt(&mut formatter);
}

#[test]
fn test_fmt_alloc_error() {
    let alloc_error = TryReserveError {
        kind: TryReserveErrorKind::AllocError {
            layout: alloc::alloc::Layout::from_size_align(0, 1).unwrap(),
        },
    };
    let mut buffer = Vec::new();
    let mut formatter = core::fmt::Formatter::new(&mut buffer);
    let _ = alloc_error.fmt(&mut formatter);
}

