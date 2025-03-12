// Answer 0

#[test]
fn test_fmt_valid_state_1() {
    let rng = Mcg128Xsl64 { state: 0 };
    let mut formatter = fmt::Formatter::new();
    rng.fmt(&mut formatter);
}

#[test]
fn test_fmt_valid_state_2() {
    let rng = Mcg128Xsl64 { state: 1u128 };
    let mut formatter = fmt::Formatter::new();
    rng.fmt(&mut formatter);
}

#[test]
fn test_fmt_valid_state_boundary_min() {
    let rng = Mcg128Xsl64 { state: u128::MIN };
    let mut formatter = fmt::Formatter::new();
    rng.fmt(&mut formatter);
}

#[test]
fn test_fmt_valid_state_boundary_max() {
    let rng = Mcg128Xsl64 { state: u128::MAX };
    let mut formatter = fmt::Formatter::new();
    rng.fmt(&mut formatter);
}

#[test]
fn test_fmt_valid_state_random() {
    let rng = Mcg128Xsl64 { state: 0x1234_5678_9abc_def0_1234_5678_9abc_def0 };
    let mut formatter = fmt::Formatter::new();
    rng.fmt(&mut formatter);
}

