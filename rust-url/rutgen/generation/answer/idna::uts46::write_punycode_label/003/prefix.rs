// Answer 0

#[test]
fn test_write_punycode_label_empty() {
    let mut sink = String::new();
    let label: &[char] = &[];
    let _ = write_punycode_label(label, &mut sink);
}

#[test]
fn test_write_punycode_label_single_ascii() {
    let mut sink = String::new();
    let label: &[char] = &['a'];
    let _ = write_punycode_label(label, &mut sink);
}

#[test]
fn test_write_punycode_label_multiple_ascii() {
    let mut sink = String::new();
    let label: &[char] = &['a', 'b', 'c'];
    let _ = write_punycode_label(label, &mut sink);
}

#[test]
fn test_write_punycode_label_unicode() {
    let mut sink = String::new();
    let label: &[char] = &['漢', '字'];
    let _ = write_punycode_label(label, &mut sink);
}

#[test]
fn test_write_punycode_label_surrogate() {
    let mut sink = String::new();
    let label: &[char] = &['😀'];
    let _ = write_punycode_label(label, &mut sink);
}

#[test]
fn test_write_punycode_label_special_characters() {
    let mut sink = String::new();
    let label: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+'];
    let _ = write_punycode_label(label, &mut sink);
}

