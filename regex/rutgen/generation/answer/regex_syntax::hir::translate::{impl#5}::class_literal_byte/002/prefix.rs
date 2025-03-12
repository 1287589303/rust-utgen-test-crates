// Answer 0

#[test]
fn test_class_literal_byte_with_ascii_character() {
    struct TestVisitor {
        pattern: &'static str,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}

        fn ast_literal_to_scalar(&self, lit: &ast::Literal) -> Result<Either<char, u8>> {
            let c = lit.c;
            if c.is_ascii() {
                Ok(Either::Left(c))
            } else {
                Err(Error {
                    kind: ErrorKind::UnicodeNotAllowed,
                    pattern: self.pattern.to_string(),
                    span: Span { start: 0, end: 0 },
                })
            }
        }
    }

    let visitor = TestVisitor { pattern: "a" };
    let literal = ast::Literal { span: Span { start: 0, end: 1 }, kind: LiteralKind::Character, c: 'a' };
    let result = visitor.class_literal_byte(&literal);
}

#[test]
fn test_class_literal_byte_with_unicode_character() {
    struct TestVisitor {
        pattern: &'static str,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}

        fn ast_literal_to_scalar(&self, lit: &ast::Literal) -> Result<Either<char, u8>> {
            let c = lit.c;
            if c.is_ascii() {
                Ok(Either::Left(c))
            } else {
                Err(Error {
                    kind: ErrorKind::UnicodeNotAllowed,
                    pattern: self.pattern.to_string(),
                    span: Span { start: 0, end: 1 },
                })
            }
        }
    }

    let visitor = TestVisitor { pattern: "é" };
    let literal = ast::Literal { span: Span { start: 0, end: 1 }, kind: LiteralKind::Character, c: 'é' };
    let result = visitor.class_literal_byte(&literal);
}

#[test]
fn test_class_literal_byte_with_byte_value() {
    struct TestVisitor {
        pattern: &'static str,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }

        fn start(&mut self) {}

        fn ast_literal_to_scalar(&self, lit: &ast::Literal) -> Result<Either<char, u8>> {
            if lit.c.is_ascii() {
                Ok(Either::Left(lit.c))
            } else {
                Ok(Either::Right(0x65)) // example byte value for 'e'
            }
        }
    }

    let visitor = TestVisitor { pattern: "byte_value" };
    let literal = ast::Literal { span: Span { start: 0, end: 1 }, kind: LiteralKind::Byte, c: 'e' };
    let result = visitor.class_literal_byte(&literal);
}

