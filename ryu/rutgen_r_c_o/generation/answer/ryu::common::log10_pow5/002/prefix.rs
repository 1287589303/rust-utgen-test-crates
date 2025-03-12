// Answer 0

#[test]
fn test_log10_pow5_e_zero() {
    let result = log10_pow5(0);
}

#[test]
fn test_log10_pow5_e_one() {
    let result = log10_pow5(1);
}

#[test]
fn test_log10_pow5_e_two_thousand_six_hundred_twenty() {
    let result = log10_pow5(2620);
}

#[test]
#[should_panic]
fn test_log10_pow5_e_two_thousand_six_hundred_twenty_one() {
    let result = log10_pow5(2621);
}

