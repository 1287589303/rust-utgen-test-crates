fn parse_perl_class(&self) -> Hir {
        let ch = self.char();
        self.bump();
        let mut class = hir::Class::new(match ch {
            'd' | 'D' => posix_class("digit").unwrap(),
            's' | 'S' => posix_class("space").unwrap(),
            'w' | 'W' => posix_class("word").unwrap(),
            unk => unreachable!("invalid Perl class \\{}", unk),
        });
        if ch.is_ascii_uppercase() {
            class.negate();
        }
        Hir::class(class)
    }