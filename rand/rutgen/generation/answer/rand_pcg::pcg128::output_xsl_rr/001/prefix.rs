// Answer 0

#[test]
fn test_output_xsl_rr_zero() {
    let state: u128 = 0;
    let _result = output_xsl_rr(state);
}

#[test]
fn test_output_xsl_rr_min_value() {
    let state: u128 = 1;
    let _result = output_xsl_rr(state);
}

#[test]
fn test_output_xsl_rr_mid_value() {
    let state: u128 = 0x80000000000000000000000000000000;
    let _result = output_xsl_rr(state);
}

#[test]
fn test_output_xsl_rr_max_value() {
    let state: u128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
    let _result = output_xsl_rr(state);
}

