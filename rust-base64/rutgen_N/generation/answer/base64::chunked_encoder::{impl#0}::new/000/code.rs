// Answer 0

#[test]
fn test_chunked_encoder_new() {
    struct MockEngine;
    
    let engine = MockEngine;
    let encoder = ChunkedEncoder::new(&engine);
    
    assert_eq!(std::ptr::addr_of!(encoder.engine), std::ptr::addr_of!(engine));
}

