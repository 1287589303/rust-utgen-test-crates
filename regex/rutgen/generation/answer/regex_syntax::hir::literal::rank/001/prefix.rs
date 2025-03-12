// Answer 0

#[test]
fn test_rank_min_value() {
    let byte: u8 = 0;
    let _ = rank(byte);
}

#[test]
fn test_rank_max_value() {
    let byte: u8 = 255;
    let _ = rank(byte);
}

#[test]
fn test_rank_mid_value() {
    let byte: u8 = 128;
    let _ = rank(byte);
}

#[test]
fn test_rank_low_value() {
    let byte: u8 = 10;
    let _ = rank(byte);
}

#[test]
fn test_rank_high_value() {
    let byte: u8 = 245;
    let _ = rank(byte);
}

