// Answer 0

#[test]
fn test_ldh_mask_boundary_case() {
    let result = ldh_mask();
}

#[test]
fn test_ldh_mask_normal_case_low() {
    let mut inputs = vec![];
    for b in 0..128 {
        inputs.push(b);
    }
    let _ = inputs.iter().map(|&b| ldh_mask());
}

#[test]
fn test_ldh_mask_normal_case_high() {
    let mut inputs = vec![127];
    let _ = inputs.iter().map(|&b| ldh_mask());
}

