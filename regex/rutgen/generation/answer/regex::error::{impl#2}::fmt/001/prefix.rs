// Answer 0

#[test]
fn test_fmt_compiled_too_big_zero() {
    let error = Error::CompiledTooBig(0);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_fmt_compiled_too_big_one() {
    let error = Error::CompiledTooBig(1);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_fmt_compiled_too_big_max_usize() {
    let error = Error::CompiledTooBig(usize::MAX);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_fmt_compiled_too_big_large_value() {
    let error = Error::CompiledTooBig(1024);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_fmt_compiled_too_big_small_value() {
    let error = Error::CompiledTooBig(10);
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

