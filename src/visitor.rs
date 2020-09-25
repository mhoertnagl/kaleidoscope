use crate::ast::*;

pub trait Visitor {
  fn visit_module(&mut self, m: &Module) {
    for s in m.statements.iter() {
      self.visit_statement(s)
    }
  }

  fn visit_statement(&mut self, s: &Statement) {
    match s {
      Statement::Let(x) => self.visit_let_statement(x),
      Statement::Def(x) => self.visit_def_statement(x),
      Statement::Assign(x) => self.visit_assign_statement(x),
      Statement::Expr(x) => self.visit_expr_statement(x),
      Statement::Return(x) => self.visit_return_statement(x),
    }
  }

  fn visit_let_statement(&mut self, s: &LetStatement) {}

  fn visit_def_statement(&mut self, s: &DefStatement) {}

  fn visit_assign_statement(&mut self, s: &AssignStatement) {}

  fn visit_expr_statement(&mut self, s: &ExprStatement) {}

  fn visit_return_statement(&mut self, s: &ReturnStatement) {}

  fn visit_expr(&mut self, e: &Expr) {
    match e {
      Expr::Atom(x) => self.visit_atom(x),
      Expr::BinOp(x) => self.visit_binop_expr(x),
      Expr::FunCall(x) => self.visit_funcall_expr(x),
    }
  }

  fn visit_binop_expr(&mut self, b: &BinaryOp) {}

  fn visit_funcall_expr(&mut self, f: &FunCall) {}

  fn visit_atom(&mut self, a: &Atom) {
    match a {
      Atom::Num(x) => self.visit_num(x),
      Atom::Id(x) => self.visit_id(x),
    }
  }

  fn visit_num(&mut self, n: f64) {}

  fn visit_id(&mut self, id: String) {}
}
