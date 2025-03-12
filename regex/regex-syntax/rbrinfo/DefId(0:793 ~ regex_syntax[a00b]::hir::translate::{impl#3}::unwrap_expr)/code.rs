fn unwrap_expr(self) -> Hir {
        match self {
            HirFrame::Expr(expr) => expr,
            HirFrame::Literal(lit) => Hir::literal(lit),
            _ => panic!("tried to unwrap expr from HirFrame, got: {:?}", self),
        }
    }