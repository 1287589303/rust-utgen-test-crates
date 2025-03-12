// Answer 0

#[test]
fn test_builder_with_utf8_enabled() {
    let builder = PikeVM::builder()
        .syntax(Config { utf8: Some(true), ..Default::default() })
        .thompson(Config { utf8: Some(true), ..Default::default() });
    let pattern = r"foo(?-u:[^b])ar.*";
    let result = builder.build(pattern);
}

#[test]
fn test_builder_with_utf8_disabled() {
    let builder = PikeVM::builder()
        .syntax(Config { utf8: Some(false), ..Default::default() })
        .thompson(Config { utf8: Some(false), ..Default::default() });
    let pattern = r"foo(?-u:[^b])ar.*";
    let result = builder.build(pattern);
}

#[test]
fn test_builder_with_small_nfa_size_limit() {
    let builder = PikeVM::builder()
        .configure(Config { nfa_size_limit: Some(Some(1)), ..Default::default() });
    let pattern = r"foo(?-u:[^b])ar.*";
    let result = builder.build(pattern);
}

#[test]
fn test_builder_with_large_nfa_size_limit() {
    let builder = PikeVM::builder()
        .configure(Config { nfa_size_limit: Some(Some(1024)), ..Default::default() });
    let pattern = r"foo(?-u:[^b])ar.*";
    let result = builder.build(pattern);
}

#[test]
fn test_builder_with_valid_utf8_input() {
    let builder = PikeVM::builder()
        .syntax(Config { utf8: Some(true), ..Default::default() });
    let pattern = r"foo(?-u:[^b])ar.*";
    let pike_vm = builder.build(pattern).unwrap();
    let (mut cache, mut caps) = (pike_vm.create_cache(), pike_vm.create_captures());
    let haystack = b"fooarzz";
    pike_vm.captures(&mut cache, haystack, &mut caps);
}

#[test]
fn test_builder_with_invalid_utf8_input() {
    let builder = PikeVM::builder()
        .syntax(Config { utf8: Some(false), ..Default::default() });
    let pattern = r"foo(?-u:[^b])ar.*";
    let pike_vm = builder.build(pattern).unwrap();
    let (mut cache, mut caps) = (pike_vm.create_cache(), pike_vm.create_captures());
    let haystack = b"\xFEfoo\xFFarzz";
    pike_vm.captures(&mut cache, haystack, &mut caps);
}

