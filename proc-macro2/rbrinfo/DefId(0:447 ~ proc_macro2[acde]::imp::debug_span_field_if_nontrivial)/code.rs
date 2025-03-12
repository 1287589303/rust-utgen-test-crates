pub(crate) fn debug_span_field_if_nontrivial(debug: &mut fmt::DebugStruct, span: Span) {
    match span {
        Span::Compiler(s) => {
            debug.field("span", &s);
        }
        Span::Fallback(s) => fallback::debug_span_field_if_nontrivial(debug, s),
    }
}