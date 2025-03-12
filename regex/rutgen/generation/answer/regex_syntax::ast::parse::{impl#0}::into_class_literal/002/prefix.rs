// Answer 0

#[test]
fn test_into_class_literal_valid_literal() {
    let span = Span { start: Position(0), end: Position(5) };
    let literal_bytes: Box<[u8]> = Box::from([97, 98, 99]); // Corresponds to "abc"
    let lit = ast::Literal { bytes: literal_bytes, exact: true };
    
    let primitive = Primitive::Literal(lit);
    
    let parser = ParserI::new(Default::default(), "abc");
    
    let _result = primitive.into_class_literal(&parser);
}

#[test]
fn test_into_class_literal_another_valid_literal() {
    let span = Span { start: Position(0), end: Position(4) };
    let literal_bytes: Box<[u8]> = Box::from([120, 121, 122]); // Corresponds to "xyz"
    let lit = ast::Literal { bytes: literal_bytes, exact: false };
    
    let primitive = Primitive::Literal(lit);
    
    let parser = ParserI::new(Default::default(), "xyz");
    
    let _result = primitive.into_class_literal(&parser);
}

#[test]
fn test_into_class_literal_empty_literal() {
    let span = Span { start: Position(0), end: Position(0) };
    let literal_bytes: Box<[u8]> = Box::from([]); // Empty literal
    let lit = ast::Literal { bytes: literal_bytes, exact: true };
    
    let primitive = Primitive::Literal(lit);
    
    let parser = ParserI::new(Default::default(), "");
    
    let _result = primitive.into_class_literal(&parser);
}

