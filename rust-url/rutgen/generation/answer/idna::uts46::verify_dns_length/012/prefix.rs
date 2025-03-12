// Answer 0

#[test]
#[should_panic]
fn test_verify_dns_length_non_ascii_case_1() {
    let domain_name = "éxample.com";
    let allow_trailing_dot = false;
    verify_dns_length(domain_name, allow_trailing_dot);
}

#[test]
#[should_panic]
fn test_verify_dns_length_non_ascii_case_2() {
    let domain_name = "пример.рф";
    let allow_trailing_dot = false;
    verify_dns_length(domain_name, allow_trailing_dot);
}

#[test]
#[should_panic]
fn test_verify_dns_length_non_ascii_case_3() {
    let domain_name = "test@domain.com";
    let allow_trailing_dot = false;
    verify_dns_length(domain_name, allow_trailing_dot);
}

#[test]
#[should_panic]
fn test_verify_dns_length_non_ascii_case_4() {
    let domain_name = "example.com😀";
    let allow_trailing_dot = false;
    verify_dns_length(domain_name, allow_trailing_dot);
}

#[test]
#[should_panic]
fn test_verify_dns_length_non_ascii_case_5() {
    let domain_name = "example.测试";
    let allow_trailing_dot = false;
    verify_dns_length(domain_name, allow_trailing_dot);
}

