fn hir_capture(&self, group: &ast::Group, expr: Hir) -> Hir {
        let (index, name) = match group.kind {
            ast::GroupKind::CaptureIndex(index) => (index, None),
            ast::GroupKind::CaptureName { ref name, .. } => {
                (name.index, Some(name.name.clone().into_boxed_str()))
            }
            // The HIR doesn't need to use non-capturing groups, since the way
            // in which the data type is defined handles this automatically.
            ast::GroupKind::NonCapturing(_) => return expr,
        };
        Hir::capture(hir::Capture { index, name, sub: Box::new(expr) })
    }