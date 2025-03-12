fn hir_dot(&self) -> Hir {
        if self.flags().dot_matches_new_line {
            Hir::class(hir::Class::new([hir::ClassRange {
                start: '\x00',
                end: '\u{10FFFF}',
            }]))
        } else if self.flags().crlf {
            Hir::class(hir::Class::new([
                hir::ClassRange { start: '\x00', end: '\x09' },
                hir::ClassRange { start: '\x0B', end: '\x0C' },
                hir::ClassRange { start: '\x0E', end: '\u{10FFFF}' },
            ]))
        } else {
            Hir::class(hir::Class::new([
                hir::ClassRange { start: '\x00', end: '\x09' },
                hir::ClassRange { start: '\x0B', end: '\u{10FFFF}' },
            ]))
        }
    }