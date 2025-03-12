fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Binary(expr) => {
                self.visit_expr_mut(&mut expr.left);
                self.visit_expr_mut(&mut expr.right);
            }
            Expr::Call(expr) => {
                self.visit_expr_mut(&mut expr.func);
                for arg in &mut expr.args {
                    self.visit_expr_mut(arg);
                }
            }
            Expr::Cast(expr) => {
                self.visit_expr_mut(&mut expr.expr);
                self.visit_type_mut(&mut expr.ty);
            }
            Expr::Field(expr) => self.visit_expr_mut(&mut expr.base),
            Expr::Index(expr) => {
                self.visit_expr_mut(&mut expr.expr);
                self.visit_expr_mut(&mut expr.index);
            }
            Expr::Paren(expr) => self.visit_expr_mut(&mut expr.expr),
            Expr::Path(expr) => self.visit_expr_path_mut(expr),
            Expr::Unary(expr) => self.visit_expr_mut(&mut expr.expr),
            _ => {}
        }
    }