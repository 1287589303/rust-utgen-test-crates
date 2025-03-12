// Answer 0

#[test]
fn test_unicode_conversion_ascii() {
    let mut idna = Idna::new(Config::default());
    let domain = "example.com";
    let mut output = String::new();
    idna.to_unicode(domain, &mut output).unwrap();
}

#[test]
fn test_unicode_conversion_non_ascii() {
    let mut idna = Idna::new(Config::default());
    let domain = "exámple.com";
    let mut output = String::new();
    idna.to_unicode(domain, &mut output).unwrap();
}

#[test]
fn test_unicode_conversion_transitional_processing() {
    let mut idna = Idna::new(Config::default().transitional_processing(true));
    let domain = "exámple.com";
    let mut output = String::new();
    idna.to_unicode(domain, &mut output).unwrap();
}

#[test]
fn test_unicode_conversion_with_hyphens() {
    let mut idna = Idna::new(Config::default().check_hyphens(true));
    let domain = "ex-ample.com";
    let mut output = String::new();
    idna.to_unicode(domain, &mut output).unwrap();
}

#[test]
fn test_unicode_conversion_failure_case() {
    let mut idna = Idna::new(Config::default().check_hyphens(true));
    let domain = "-example.com";
    let mut output = String::new();
    assert!(idna.to_unicode(domain, &mut output).is_err());
}

#[test]
fn test_unicode_conversion_std3_rules() {
    let mut idna = Idna::new(Config::default().use_std3_ascii_rules(true));
    let domain = "ex@ample.com";
    let mut output = String::new();
    assert!(idna.to_unicode(domain, &mut output).is_err());
}

