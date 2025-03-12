// Answer 0

#[test]
fn test_ast_literal_to_scalar_success_case() {
    struct TestVisitor {
        output: Result<Either<char, u8>, Error>,
    }

    impl Visitor for TestVisitor {
        type Output = Result<Either<char, u8>, Error>;
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }
        
        fn start(&mut self) {}
    }

    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Default::default()
        }),
        utf8: false,
        line_terminator: b'\n',
    };

    let span = Span {
        start: Position::new(0),
        end: Position::new(1),
    };

    let lit = ast::Literal {
        span,
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::X),
        c: '', // char corresponding to byte 0x7F
    };

    let translator_i = TranslatorI::new(&trans, "test pattern");

    let result = translator_i.ast_literal_to_scalar(&lit);
}

