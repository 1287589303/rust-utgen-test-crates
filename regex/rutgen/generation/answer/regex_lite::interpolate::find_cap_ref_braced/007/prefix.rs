// Answer 0

#[test]
fn test_find_cap_ref_braced_with_number() {
    let rep = &[b'{', b'0', b'}'];
    let i = 1;
    let result = find_cap_ref_braced(rep, i);
}

#[test]
fn test_find_cap_ref_braced_with_named() {
    let rep = &[b'{', b'a', b'b', b'}'];
    let i = 1;
    let result = find_cap_ref_braced(rep, i);
}

#[test]
fn test_find_cap_ref_braced_with_multiple_digits() {
    let rep = &[b'{', b'1', b'2', b'3', b'}'];
    let i = 1;
    let result = find_cap_ref_braced(rep, i);
}

#[test]
fn test_find_cap_ref_braced_with_underscore() {
    let rep = &[b'{', b'f', b'o', b'o', b'_', b'1', b'}'];
    let i = 1;
    let result = find_cap_ref_braced(rep, i);
}

#[test]
fn test_find_cap_ref_braced_with_non_alpha_numeric() {
    let rep = &[b'{', b'@', b'1', b'}'];
    let i = 1;
    let result = find_cap_ref_braced(rep, i);
}

