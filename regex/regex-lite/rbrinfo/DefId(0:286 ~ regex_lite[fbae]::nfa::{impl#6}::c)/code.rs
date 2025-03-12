fn c(&self, hir: &Hir) -> Result<ThompsonRef, Error> {
        match *hir.kind() {
            HirKind::Empty => self.c_empty(),
            HirKind::Char(ch) => self.c_char(ch),
            HirKind::Class(ref class) => self.c_class(class),
            HirKind::Look(ref look) => self.c_look(look),
            HirKind::Repetition(ref rep) => self.c_repetition(rep),
            HirKind::Capture(ref cap) => {
                self.c_capture(cap.index, cap.name.as_deref(), &cap.sub)
            }
            HirKind::Concat(ref subs) => {
                self.c_concat(subs.iter().map(|s| self.c(s)))
            }
            HirKind::Alternation(ref subs) => {
                self.c_alternation(subs.iter().map(|s| self.c(s)))
            }
        }
    }