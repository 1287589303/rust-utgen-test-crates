// Answer 0

#[test]
fn test_check_url_code_point_invalid_percent_encoding() {
    let invalid_input = Input { chars: "%G1".chars() }; // 'G' is not a valid ASCII hex digit
    check_url_code_point(|violation| {
        // This closure will be triggered for SyntaxViolation::PercentDecode
    }, '%', &invalid_input);
}

#[test]
fn test_check_url_code_point_non_url_code_point_before() {
    let invalid_input = Input { chars: "&%20".chars() }; // '&' is a valid char but we want to ensure the point is tested
    check_url_code_point(|violation| {
        // This closure will be triggered for SyntaxViolation::NonUrlCodePoint
    }, '%', &invalid_input);
}

#[test]
fn test_check_url_code_point_non_url_code_point_after() {
    let invalid_input = Input { chars: "%@2".chars() }; // '@' is not a valid URL code point here
    check_url_code_point(|violation| {
        // This closure will be triggered for SyntaxViolation::NonUrlCodePoint
    }, '%', &invalid_input);
}

