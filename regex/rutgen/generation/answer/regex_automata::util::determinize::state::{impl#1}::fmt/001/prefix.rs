// Answer 0

#[test]
fn test_fmt_with_non_empty_state() {
    let data = Arc::new([1u8, 2, 3]);
    let state = State(data.clone());
    let _ = core::fmt::format(format_args!("{:?}", state));
}

#[test]
fn test_fmt_with_another_non_empty_state() {
    let data = Arc::new([0u8, 255, 128, 64]);
    let state = State(data.clone());
    let _ = core::fmt::format(format_args!("{:?}", state));
}

