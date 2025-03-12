// Answer 0

#[should_panic]
fn test_log10_pow5_negative_input() {
    let e: i32 = -1;
    let _ = log10_pow5(e);
}

#[should_panic]
fn test_log10_pow5_negative_boundary() {
    let e: i32 = -2620;
    let _ = log10_pow5(e);
}

