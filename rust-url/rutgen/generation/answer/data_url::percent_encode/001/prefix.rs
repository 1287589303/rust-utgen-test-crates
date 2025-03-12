// Answer 0

#[test]
fn test_percent_encode_zero() {
    let mut output = String::new();
    percent_encode(0, &mut output);
}

#[test]
fn test_percent_encode_max() {
    let mut output = String::new();
    percent_encode(255, &mut output);
}

#[test]
fn test_percent_encode_mid_range() {
    let mut output = String::new();
    percent_encode(128, &mut output);
}

#[test]
fn test_percent_encode_boundary_low() {
    let mut output = String::new();
    percent_encode(1, &mut output);
}

#[test]
fn test_percent_encode_boundary_high() {
    let mut output = String::new();
    percent_encode(254, &mut output);
}

