// Answer 0

#[test]
fn test_to_ascii_passthrough_ascii() {
    let uts46 = Uts46::new();
    let domain_name = b"example.com";
    let ascii_deny_list = AsciiDenyList { bits: 0 };
    let hyphens = Hyphens::Allow;
    let dns_length = DnsLength::Ignore;

    let result = uts46.to_ascii(domain_name, ascii_deny_list, hyphens, dns_length);
}

#[test]
fn test_to_ascii_passthrough_unicode() {
    let uts46 = Uts46::new();
    let domain_name = "exámplé.com".as_bytes(); // valid UTF-8 non-ASCII
    let ascii_deny_list = AsciiDenyList { bits: 0 };
    let hyphens = Hyphens::Allow;
    let dns_length = DnsLength::VerifyAllowRootDot;

    let result = uts46.to_ascii(domain_name, ascii_deny_list, hyphens, dns_length);
}

#[test]
fn test_to_ascii_wrote_to_sink() {
    let uts46 = Uts46::new();
    let domain_name = b"example-with-hyphen.com";
    let ascii_deny_list = AsciiDenyList { bits: 0 };
    let hyphens = Hyphens::Allow;
    let dns_length = DnsLength::Ignore;

    let result = uts46.to_ascii(domain_name, ascii_deny_list, hyphens, dns_length);
} 

#[test]
#[should_panic]
fn test_to_ascii_invalid() {
    let uts46 = Uts46::new();
    let domain_name = b"invalid\xFFdomain.com"; // invalid UTF-8
    let ascii_deny_list = AsciiDenyList { bits: 0 };
    let hyphens = Hyphens::Allow;
    let dns_length = DnsLength::VerifyAllowRootDot;

    let result = uts46.to_ascii(domain_name, ascii_deny_list, hyphens, dns_length);
}

