pub fn capture(capture: Capture) -> Hir {
        let props = Properties::capture(&capture);
        Hir { kind: HirKind::Capture(capture), props }
    }