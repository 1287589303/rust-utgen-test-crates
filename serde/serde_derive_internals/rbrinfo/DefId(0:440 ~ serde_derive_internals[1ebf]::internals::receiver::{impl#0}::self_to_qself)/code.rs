fn self_to_qself(&self, qself: &mut Option<QSelf>, path: &mut Path) {
        if path.leading_colon.is_some() || path.segments[0].ident != "Self" {
            return;
        }

        if path.segments.len() == 1 {
            self.self_to_expr_path(path);
            return;
        }

        let span = path.segments[0].ident.span();
        *qself = Some(QSelf {
            lt_token: Token![<](span),
            ty: Box::new(Type::Path(self.self_ty(span))),
            position: 0,
            as_token: None,
            gt_token: Token![>](span),
        });

        path.leading_colon = Some(**path.segments.pairs().next().unwrap().punct().unwrap());

        let segments = mem::take(&mut path.segments);
        path.segments = segments.into_pairs().skip(1).collect();
    }