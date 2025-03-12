fn c(&self, expr: &Hir) -> Result<ThompsonRef, BuildError> {
        use regex_syntax::hir::{Class, HirKind::*};

        match *expr.kind() {
            Empty => self.c_empty(),
            Literal(hir::Literal(ref bytes)) => self.c_literal(bytes),
            Class(Class::Bytes(ref c)) => self.c_byte_class(c),
            Class(Class::Unicode(ref c)) => self.c_unicode_class(c),
            Look(ref look) => self.c_look(look),
            Repetition(ref rep) => self.c_repetition(rep),
            Capture(ref c) => self.c_cap(c.index, c.name.as_deref(), &c.sub),
            Concat(ref es) => self.c_concat(es.iter().map(|e| self.c(e))),
            Alternation(ref es) => self.c_alt_slice(es),
        }
    }