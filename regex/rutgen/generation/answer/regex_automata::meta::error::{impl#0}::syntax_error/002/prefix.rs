// Answer 0

#[test]
fn test_syntax_error_valid() {
    use regex_syntax::Error as RegexError;
    use crate::{PatternID, BuildError, BuildErrorKind};

    let pattern_id = PatternID::new(1); 
    let regex_error = RegexError::new("syntax error", 0); 
    let build_error = BuildError {
        kind: BuildErrorKind::Syntax {
            pid: pattern_id,
            err: regex_error,
        },
    };

    let _result = build_error.syntax_error();
}

#[test]
fn test_syntax_error_another_valid() {
    use regex_syntax::Error as RegexError;
    use crate::{PatternID, BuildError, BuildErrorKind};

    let pattern_id = PatternID::new(2); 
    let regex_error = RegexError::new("another syntax error", 1); 
    let build_error = BuildError {
        kind: BuildErrorKind::Syntax {
            pid: pattern_id,
            err: regex_error,
        },
    };

    let _result = build_error.syntax_error();
}

