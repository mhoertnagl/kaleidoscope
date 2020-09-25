use crate::ast::*;

pub trait Visitor {
  fn visit_module(&mut self, m: &Module) {

  }

  fn visit_statement(&mut self, s: &Statement) {
    // match s {
    //   Statement::Let(l) => self.visit_let_statement(l)
    // }
  }

  fn visit_let_statement(&mut self, l: &LetStatement) {

  }

  fn visit_expr(&mut self, e: &Expr) {

  }

  // fn visit_binary_op(&mut self, b: &BinaryOp) {
  // }

  // fn visit_function_call(&mut self, f: &FunCall) {
  // }

  fn visit_atom(&mut self, atom: &Atom) {}
}