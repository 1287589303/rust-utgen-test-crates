// Answer 0

#[test]
fn test_fmt_class_ascii_alpha_negated() {
    struct MyVisitor {
        output: String,
    }

    impl Visitor for MyVisitor {
        type Output = String;
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }
        
        fn start(&mut self) {}
    }

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Alpha,
        negated: true,
    };

    let mut writer = Writer { wtr: &mut String::new() };
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_alpha_not_negated() {
    struct MyVisitor {
        output: String,
    }

    impl Visitor for MyVisitor {
        type Output = String;
        type Err = ();

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(self.output)
        }

        fn start(&mut self) {}
    }

    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Alpha,
        negated: false,
    };

    let mut writer = Writer { wtr: &mut String::new() };
    writer.fmt_class_ascii(&ast).unwrap();
}

