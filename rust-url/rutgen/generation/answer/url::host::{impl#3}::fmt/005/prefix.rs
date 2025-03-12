// Answer 0

#[test]
fn test_fmt_domain_ascii() {
    let host = Host::Domain(String::from("example.com"));
    let mut output = String::new();
    let _ = write!(&mut output, "{}", host);
}

#[test]
fn test_fmt_domain_idna() {
    let host = Host::Domain(String::from("xn--ls8h.XN--LS8H"));
    let mut output = String::new();
    let _ = write!(&mut output, "{}", host);
}

#[test]
fn test_fmt_domain_edge_case_long_label() {
    let host = Host::Domain(String::from("a".repeat(63) + ".com"));
    let mut output = String::new();
    let _ = write!(&mut output, "{}", host);
}

#[test]
fn test_fmt_domain_edge_case_total_length() {
    let host = Host::Domain(String::from("a".repeat(253 - 4) + ".com")); // 253 total length
    let mut output = String::new();
    let _ = write!(&mut output, "{}", host);
}

#[test]
fn test_fmt_domain_special_characters() {
    let host = Host::Domain(String::from("ex-ample.com"));
    let mut output = String::new();
    let _ = write!(&mut output, "{}", host);
}

