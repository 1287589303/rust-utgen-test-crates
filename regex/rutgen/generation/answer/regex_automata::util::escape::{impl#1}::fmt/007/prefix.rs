// Answer 0

#[test]
fn test_debug_haystack_with_null_byte() {
    let haystack = DebugHaystack(&[0]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_debug_haystack_with_control_character_b() {
    let haystack = DebugHaystack(&[11]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_debug_haystack_with_control_character_c() {
    let haystack = DebugHaystack(&[12]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_debug_haystack_with_control_character_tab() {
    let haystack = DebugHaystack(&[9]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_debug_haystack_with_control_character_newline() {
    let haystack = DebugHaystack(&[10]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_debug_haystack_with_control_character_carriage_return() {
    let haystack = DebugHaystack(&[13]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

#[test]
fn test_debug_haystack_with_control_character_range() {
    let haystack = DebugHaystack(&[1, 2, 3, 4, 5, 6, 7, 8, 14, 15, 16, 17, 18, 19, 127]);
    let _ = core::fmt::write(&mut core::fmt::Formatter::new(), |f| haystack.fmt(f));
}

