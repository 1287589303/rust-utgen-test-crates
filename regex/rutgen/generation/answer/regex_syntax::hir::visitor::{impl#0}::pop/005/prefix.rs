// Answer 0

#[test]
fn test_pop_frame_capture() {
    struct DummyRepetition;
    struct DummyCapture;

    let capture = DummyCapture;
    let frame = Frame::Capture(&capture);
    let visitor = HeapVisitor::new();

    let result = visitor.pop(frame);
}

#[test]
fn test_pop_frame_capture_with_different_capture() {
    struct AnotherDummyCapture;

    let another_capture = AnotherDummyCapture;
    let frame = Frame::Capture(&another_capture);
    let visitor = HeapVisitor::new();

    let result = visitor.pop(frame);
}

