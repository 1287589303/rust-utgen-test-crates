// Answer 0

#[test]
fn test_pop_repetition() {
    struct DummyRepetition;

    let frame = Frame::Repetition(&DummyRepetition);
    let visitor = HeapVisitor::new();
    let result = visitor.pop(frame);
}

#[test]
fn test_pop_repetition_empty() {
    struct DummyRepetition;

    let frame = Frame::Repetition(&DummyRepetition);
    let visitor = HeapVisitor::new();
    let result = visitor.pop(frame);
}

