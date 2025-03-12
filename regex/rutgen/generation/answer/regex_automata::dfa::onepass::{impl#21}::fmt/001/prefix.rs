// Answer 0

#[test]
fn test_fmt_not_one_pass_with_invalid_pattern() {
    let build_error = BuildError {
        kind: BuildErrorKind::NotOnePass {
            msg: "invalid pattern"
        },
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", build_error);
}

#[test]
fn test_fmt_not_one_pass_with_empty_message() {
    let build_error = BuildError {
        kind: BuildErrorKind::NotOnePass {
            msg: ""
        },
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", build_error);
}

#[test]
fn test_fmt_not_one_pass_with_long_message() {
    let build_error = BuildError {
        kind: BuildErrorKind::NotOnePass {
            msg: "a very long pattern string that exceeds typical lengths for regex patterns"
        },
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", build_error);
}

