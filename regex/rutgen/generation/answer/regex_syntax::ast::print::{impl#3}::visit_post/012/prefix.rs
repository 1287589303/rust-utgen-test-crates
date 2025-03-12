// Answer 0

#[test]
fn test_visit_post_empty() {
    let ast = Ast::Empty(Box::new(Span::default()));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_flags() {
    let ast = Ast::Flags(Box::new(SetFlags {
        span: Span::default(),
        flags: Flags::default(),
    }));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_literal() {
    let ast = Ast::Literal(Box::new(Literal {
        span: Span::default(),
        kind: LiteralKind::Verbatim,
        c: 'a',
    }));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_dot() {
    let ast = Ast::Dot(Box::new(Span::default()));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_assertion() {
    let ast = Ast::Assertion(Box::new(Assertion {
        span: Span::default(),
        kind: AssertionKind::StartLine,
    }));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_class_perl() {
    let ast = Ast::ClassPerl(Box::new(ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Digit,
        negated: false,
    }));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_class_unicode() {
    let ast = Ast::ClassUnicode(Box::new(ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::OneLetter('a'),
    }));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_class_bracketed() {
    let ast = Ast::ClassBracketed(Box::new(ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ClassSet::Normal,
    }));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_repetition() {
    let ast = Ast::Repetition(Box::new(Repetition {
        span: Span::default(),
        op: RepetitionOp::ZeroOrMore,
        greedy: true,
        ast: Box::new(Ast::Empty(Box::new(Span::default()))),
    }));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_group() {
    let ast = Ast::Group(Box::new(Group {
        span: Span::default(),
        kind: GroupKind::Capture,
        ast: Box::new(Ast::Empty(Box::new(Span::default()))),
    }));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_alternation() {
    let ast = Ast::Alternation(Box::new(Alternation {
        span: Span::default(),
        asts: Vec::new(),
    }));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_concat() {
    let ast = Ast::Concat(Box::new(Concat {
        span: Span::default(),
        asts: Vec::new(),
    }));
    let mut writer = Writer { wtr: Vec::new() };
    writer.visit_post(&ast).unwrap();
}

