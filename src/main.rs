mod lexer;
mod token;
mod ast;

#[macro_use] extern crate lalrpop_util;

use crate::ast::{
  Module,
  Statement,
  LetStatement,
  DefStatement,
  AssignStatement,
  ExprStatement,
  ReturnStatement,
  Expr,
  Atom,
  Opcode
};

fn main() {
  let prog = r"
    let a = 0;
    let b = 1;
    def f(x, y) begin
      let z = x * y;
      return z - 3 * x;
    end
    h = 3 * (a + f(b, 2));
    h;
  ";
  match parser::ModuleParser::new().parse(prog) {
    Ok(v) => println!("{}", v),
    Err(err) => println!("{}", err),
  }
}

lalrpop_mod!(pub parser);

#[test]
fn parser() {
  // let exp = Module {
  //   statements: vec![
  //     Statement::Expr(ExprStatement::new())
  //   ]
  // };

  let act = parser::ModuleParser::new()
    .parse("(1 + 2) * 4;")
    .unwrap();

  assert_eq!(&format!("{}", act), "((1 + 2) * 4);\n");
}