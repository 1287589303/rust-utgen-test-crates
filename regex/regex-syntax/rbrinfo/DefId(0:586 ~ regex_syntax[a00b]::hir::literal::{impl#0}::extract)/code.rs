pub fn extract(&self, hir: &Hir) -> Seq {
        use crate::hir::HirKind::*;

        match *hir.kind() {
            Empty | Look(_) => Seq::singleton(self::Literal::exact(vec![])),
            Literal(hir::Literal(ref bytes)) => {
                let mut seq =
                    Seq::singleton(self::Literal::exact(bytes.to_vec()));
                self.enforce_literal_len(&mut seq);
                seq
            }
            Class(hir::Class::Unicode(ref cls)) => {
                self.extract_class_unicode(cls)
            }
            Class(hir::Class::Bytes(ref cls)) => self.extract_class_bytes(cls),
            Repetition(ref rep) => self.extract_repetition(rep),
            Capture(hir::Capture { ref sub, .. }) => self.extract(sub),
            Concat(ref hirs) => match self.kind {
                ExtractKind::Prefix => self.extract_concat(hirs.iter()),
                ExtractKind::Suffix => self.extract_concat(hirs.iter().rev()),
            },
            Alternation(ref hirs) => {
                // Unlike concat, we always union starting from the beginning,
                // since the beginning corresponds to the highest preference,
                // which doesn't change based on forwards vs reverse.
                self.extract_alternation(hirs.iter())
            }
        }
    }