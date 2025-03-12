// Answer 0

#[test]
fn test_match_error_fmt_quit() {
    let byte = 0x1; 
    let offset = 10; 
    let match_error = MatchError::quit(byte, offset);
    let _ = format!("{}", match_error);
}

#[test]
fn test_match_error_fmt_gave_up() {
    let offset = 20; 
    let match_error = MatchError::gave_up(offset);
    let _ = format!("{}", match_error);
}

#[test]
fn test_match_error_fmt_haystack_too_long() {
    let length = 1000; 
    let match_error = MatchError::haystack_too_long(length);
    let _ = format!("{}", match_error);
}

#[test]
fn test_match_error_fmt_unsupported_anchored_yes() {
    let mode = Anchored::Yes; 
    let match_error = MatchError::unsupported_anchored(mode);
    let _ = format!("{}", match_error);
}

#[test]
fn test_match_error_fmt_unsupported_anchored_no() {
    let mode = Anchored::No; 
    let match_error = MatchError::unsupported_anchored(mode);
    let _ = format!("{}", match_error);
}

#[test]
fn test_match_error_fmt_unsupported_anchored_pattern() {
    let pid = PatternID(0); 
    let mode = Anchored::Pattern(pid); 
    let match_error = MatchError::unsupported_anchored(mode);
    let _ = format!("{}", match_error);
}

