// Answer 0

#[test]
fn test_fmt_compiled_too_big_with_min_limit() {
    let error = Error::CompiledTooBig(1);
    let mut buffer = alloc::string::String::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_compiled_too_big_with_mid_limit() {
    let error = Error::CompiledTooBig(100);
    let mut buffer = alloc::string::String::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_compiled_too_big_with_max_limit() {
    let error = Error::CompiledTooBig(usize::MAX);
    let mut buffer = alloc::string::String::new();
    let _ = error.fmt(&mut buffer);
}

