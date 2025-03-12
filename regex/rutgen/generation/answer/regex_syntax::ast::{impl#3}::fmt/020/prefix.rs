// Answer 0

#[test]
fn test_flag_repeated_negation_valid_span_0_1() {
    let original = Span { start: Position(0), end: Position(1) };
    let error_kind = ErrorKind::FlagRepeatedNegation { original };
    let _ = format!("{}", error_kind);
}

#[test]
fn test_flag_repeated_negation_valid_span_1_1() {
    let original = Span { start: Position(1), end: Position(1) };
    let error_kind = ErrorKind::FlagRepeatedNegation { original };
    let _ = format!("{}", error_kind);
}

#[test]
fn test_flag_repeated_negation_valid_span_0_0() {
    let original = Span { start: Position(0), end: Position(0) };
    let error_kind = ErrorKind::FlagRepeatedNegation { original };
    let _ = format!("{}", error_kind);
}

#[test]
fn test_flag_repeated_negation_edge_case_span_u32_max_minus_1() {
    let original = Span { start: Position(u32::MAX - 1), end: Position(u32::MAX) };
    let error_kind = ErrorKind::FlagRepeatedNegation { original };
    let _ = format!("{}", error_kind);
}

#[test]
fn test_flag_repeated_negation_edge_case_span_u32_max() {
    let original = Span { start: Position(u32::MAX), end: Position(u32::MAX) };
    let error_kind = ErrorKind::FlagRepeatedNegation { original };
    let _ = format!("{}", error_kind);
}

