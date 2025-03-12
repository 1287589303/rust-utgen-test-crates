// Answer 0

#[test]
fn test_input_debug_format_non_empty_haystack() {
    let haystack: &[u8] = b"test haystack";
    let span = Span { start: 0, end: 4 };
    let anchored = Anchored::No;
    let earliest = true;

    let input = Input::new(haystack)
        .span(span)
        .anchored(anchored)
        .earliest(earliest);

    core::fmt::Debug::fmt(&input, &mut core::fmt::Formatter::new());
}

#[test]
fn test_input_debug_format_with_anchored_yes() {
    let haystack: &[u8] = b"another test";
    let span = Span { start: 3, end: 7 };
    let anchored = Anchored::Yes;
    let earliest = false;

    let input = Input::new(haystack)
        .span(span)
        .anchored(anchored)
        .earliest(earliest);

    core::fmt::Debug::fmt(&input, &mut core::fmt::Formatter::new());
}

#[test]
fn test_input_debug_format_with_pattern() {
    let haystack: &[u8] = b"pattern match example";
    let span = Span { start: 0, end: 8 };
    let anchored = Anchored::Pattern(PatternID(1));
    let earliest = true;

    let input = Input::new(haystack)
        .span(span)
        .anchored(anchored)
        .earliest(earliest);

    core::fmt::Debug::fmt(&input, &mut core::fmt::Formatter::new());
}

#[test]
fn test_input_debug_format_boundary_case() {
    let haystack: &[u8] = b"";
    let span = Span { start: 0, end: 0 };
    let anchored = Anchored::No;
    let earliest = false;

    let input = Input::new(haystack)
        .span(span)
        .anchored(anchored)
        .earliest(earliest);

    core::fmt::Debug::fmt(&input, &mut core::fmt::Formatter::new());
}

