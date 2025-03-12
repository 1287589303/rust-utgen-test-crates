// Answer 0

#[test]
fn test_accel_new() {
    let accel = Accel::new();
    let expected_bytes = [0; ACCEL_CAP];
    assert_eq!(accel.bytes, expected_bytes);
}

