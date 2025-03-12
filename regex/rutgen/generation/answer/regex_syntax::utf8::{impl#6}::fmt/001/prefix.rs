// Answer 0

#[test]
fn test_scalar_range_fmt_start_0_end_0() {
    let range = ScalarRange { start: 0, end: 0 };
    let _ = format!("{:?}", range);
}

#[test]
fn test_scalar_range_fmt_start_0_end_max() {
    let range = ScalarRange { start: 0, end: 0x10FFFF };
    let _ = format!("{:?}", range);
}

#[test]
fn test_scalar_range_fmt_start_max_end_max() {
    let range = ScalarRange { start: 0x10FFFF, end: 0x10FFFF };
    let _ = format!("{:?}", range);
}

#[test]
fn test_scalar_range_fmt_start_0_end_1() {
    let range = ScalarRange { start: 0x000000, end: 0x000001 };
    let _ = format!("{:?}", range);
}

#[test]
fn test_scalar_range_fmt_start_large_end_max() {
    let range = ScalarRange { start: 0xFFFFFF, end: 0x10FFFF };
    let _ = format!("{:?}", range);
}

