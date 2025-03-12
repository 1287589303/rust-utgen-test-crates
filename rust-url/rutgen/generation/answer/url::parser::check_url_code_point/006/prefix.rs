// Answer 0

#[test]
fn test_check_url_code_point_percent_decode_valid() {
    let input = Input {
        chars: "%%01".chars(), // '%' followed by two valid ASCII hex digits '01'
    };
    let mock_violation_fn = |violation: SyntaxViolation| {
        panic!("Expected no violation, received: {:?}", violation);
    };
    
    check_url_code_point(&mock_violation_fn, '%', &input);
}

#[test]
fn test_check_url_code_point_non_hex_after_percent() {
    let input = Input {
        chars: "%GZ".chars(), // '%' followed by non-hex characters 'G' and 'Z'
    };
    let mock_violation_fn = |violation: SyntaxViolation| { 
        assert_eq!(violation, SyntaxViolation::PercentDecode); 
    };
    
    check_url_code_point(&mock_violation_fn, '%', &input);
}

#[test]
fn test_check_url_code_point_non_url_code_point() {
    let input = Input {
        chars: "A%12".chars(), // 'A' is a valid URL code point and '%' followed by valid ASCII hex digits
    };
    let mock_violation_fn = |violation: SyntaxViolation| {
        panic!("Expected no violation, received: {:?}", violation);
    };
    
    check_url_code_point(&mock_violation_fn, 'A', &input);
}

