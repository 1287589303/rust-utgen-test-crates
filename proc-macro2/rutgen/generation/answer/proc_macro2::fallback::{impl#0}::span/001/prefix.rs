// Answer 0

#[test]
fn test_lex_error_span_compiler() {
    let span = Span::Compiler(proc_macro::Span::call_site());
    let lex_error = LexError { span };
    let returned_span = lex_error.span();
}

#[test]
fn test_lex_error_span_fallback() {
    let span = Span::Fallback(Span { lo: 0, hi: 10 });
    let lex_error = LexError { span };
    let returned_span = lex_error.span();
}

#[test]
fn test_lex_error_span_fallback_boundaries() {
    let span_low = Span::Fallback(Span { lo: u32::MIN, hi: u32::MIN });
    let lex_error_low = LexError { span: span_low };
    let returned_span_low = lex_error_low.span();

    let span_high = Span::Fallback(Span { lo: u32::MAX, hi: u32::MAX });
    let lex_error_high = LexError { span: span_high };
    let returned_span_high = lex_error_high.span();
}

