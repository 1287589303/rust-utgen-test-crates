// Answer 0

#[test]
fn test_unexpected_with_seq() {
    let content = Content::Seq(vec![
        Content::Bool(true),
        Content::U8(0),
        Content::F32(3.14),
        Content::Char('a'),
    ]);
    content.unexpected();
}

