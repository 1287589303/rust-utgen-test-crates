// Answer 0

#[test]
fn test_check_url_code_point_valid_hex() {
    struct MockSyntaxViolation;

    let vfn = |_: MockSyntaxViolation| {};
    let c = 'A'; // c is not '%'
    let input = Input { chars: "01".chars() }; // two ASCII hex digits '0' and '1'

    check_url_code_point(&vfn, c, &input);
}

#[test]
fn test_check_url_code_point_valid_hex_uppercase() {
    struct MockSyntaxViolation;

    let vfn = |_: MockSyntaxViolation| {};
    let c = 'F'; // c is not '%'
    let input = Input { chars: "AB".chars() }; // two ASCII hex digits 'A' and 'B'

    check_url_code_point(&vfn, c, &input);
}

#[test]
fn test_check_url_code_point_valid_hex_mixed() {
    struct MockSyntaxViolation;

    let vfn = |_: MockSyntaxViolation| {};
    let c = '2'; // c is not '%'
    let input = Input { chars: "34".chars() }; // two ASCII hex digits '3' and '4'

    check_url_code_point(&vfn, c, &input);
}

