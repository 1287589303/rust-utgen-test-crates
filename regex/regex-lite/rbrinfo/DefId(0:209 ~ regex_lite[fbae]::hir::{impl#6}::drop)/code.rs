fn drop(&mut self) {
        use core::mem;

        match *self.kind() {
            HirKind::Empty
            | HirKind::Char(_)
            | HirKind::Class(_)
            | HirKind::Look(_) => return,
            HirKind::Capture(ref x) if x.sub.kind.subs().is_empty() => return,
            HirKind::Repetition(ref x) if x.sub.kind.subs().is_empty() => {
                return
            }
            HirKind::Concat(ref x) if x.is_empty() => return,
            HirKind::Alternation(ref x) if x.is_empty() => return,
            _ => {}
        }

        let mut stack = vec![mem::replace(self, Hir::empty())];
        while let Some(mut expr) = stack.pop() {
            match expr.kind {
                HirKind::Empty
                | HirKind::Char(_)
                | HirKind::Class(_)
                | HirKind::Look(_) => {}
                HirKind::Capture(ref mut x) => {
                    stack.push(mem::replace(&mut x.sub, Hir::empty()));
                }
                HirKind::Repetition(ref mut x) => {
                    stack.push(mem::replace(&mut x.sub, Hir::empty()));
                }
                HirKind::Concat(ref mut x) => {
                    stack.extend(x.drain(..));
                }
                HirKind::Alternation(ref mut x) => {
                    stack.extend(x.drain(..));
                }
            }
        }
    }