// Answer 0

#[test]
fn test_apply_ascii_deny_list_with_a() {
    let b: u8 = 65; // 'A'
    let deny_list: u128 = 0; // (deny_list & (1u128 << 65)) == 0
    let result = apply_ascii_deny_list_to_potentially_upper_case_ascii(b, deny_list);
    // result should be 'a'
}

#[test]
fn test_apply_ascii_deny_list_with_b() {
    let b: u8 = 66; // 'B'
    let deny_list: u128 = 0; // (deny_list & (1u128 << 66)) == 0
    let result = apply_ascii_deny_list_to_potentially_upper_case_ascii(b, deny_list);
    // result should be 'b'
}

#[test]
fn test_apply_ascii_deny_list_with_z() {
    let b: u8 = 90; // 'Z'
    let deny_list: u128 = 0; // (deny_list & (1u128 << 90)) == 0
    let result = apply_ascii_deny_list_to_potentially_upper_case_ascii(b, deny_list);
    // result should be 'z'
}

#[test]
fn test_apply_ascii_deny_list_with_middle_case() {
    let b: u8 = 78; // 'N'
    let deny_list: u128 = 0; // (deny_list & (1u128 << 78)) == 0
    let result = apply_ascii_deny_list_to_potentially_upper_case_ascii(b, deny_list);
    // result should be 'n'
}

