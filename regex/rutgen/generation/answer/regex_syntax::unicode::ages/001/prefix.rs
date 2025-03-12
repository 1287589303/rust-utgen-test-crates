// Answer 0

#[test]
fn test_ages_valid_v1_1() {
    ages("V1_1").ok();
}

#[test]
fn test_ages_valid_v2_0() {
    ages("V2_0").ok();
}

#[test]
fn test_ages_valid_v2_1() {
    ages("V2_1").ok();
}

#[test]
fn test_ages_valid_v3_0() {
    ages("V3_0").ok();
}

#[test]
fn test_ages_valid_v3_1() {
    ages("V3_1").ok();
}

#[test]
fn test_ages_valid_v3_2() {
    ages("V3_2").ok();
}

#[test]
fn test_ages_valid_v4_0() {
    ages("V4_0").ok();
}

#[test]
fn test_ages_valid_v4_1() {
    ages("V4_1").ok();
}

#[test]
fn test_ages_valid_v5_0() {
    ages("V5_0").ok();
}

#[test]
fn test_ages_valid_v5_1() {
    ages("V5_1").ok();
}

#[test]
fn test_ages_valid_v5_2() {
    ages("V5_2").ok();
}

#[test]
fn test_ages_valid_v6_0() {
    ages("V6_0").ok();
}

#[test]
fn test_ages_valid_v6_1() {
    ages("V6_1").ok();
}

#[test]
fn test_ages_valid_v6_2() {
    ages("V6_2").ok();
}

#[test]
fn test_ages_valid_v6_3() {
    ages("V6_3").ok();
}

#[test]
fn test_ages_valid_v7_0() {
    ages("V7_0").ok();
}

#[test]
fn test_ages_valid_v8_0() {
    ages("V8_0").ok();
}

#[test]
fn test_ages_valid_v9_0() {
    ages("V9_0").ok();
}

#[test]
fn test_ages_valid_v10_0() {
    ages("V10_0").ok();
}

#[test]
fn test_ages_valid_v11_0() {
    ages("V11_0").ok();
}

#[test]
fn test_ages_valid_v12_0() {
    ages("V12_0").ok();
}

#[test]
fn test_ages_valid_v12_1() {
    ages("V12_1").ok();
}

#[test]
fn test_ages_valid_v13_0() {
    ages("V13_0").ok();
}

#[test]
fn test_ages_valid_v14_0() {
    ages("V14_0").ok();
}

#[test]
fn test_ages_valid_v15_0() {
    ages("V15_0").ok();
}

#[test]
fn test_ages_valid_v15_1() {
    ages("V15_1").ok();
}

#[test]
fn test_ages_valid_v16_0() {
    ages("V16_0").ok();
}

#[test]
fn test_ages_invalid_empty() {
    assert!(ages("").is_err());
}

#[test]
fn test_ages_invalid_unknown() {
    assert!(ages("invalid_age").is_err());
}

#[test]
fn test_ages_invalid_v17_0() {
    assert!(ages("V17_0").is_err());
}

#[test]
fn test_ages_boundary_v1_1() {
    let result = ages("V1_1");
    assert!(result.is_ok());
}

#[test]
fn test_ages_boundary_v16_0() {
    let result = ages("V16_0");
    assert!(result.is_ok());
}

