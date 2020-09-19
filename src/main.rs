mod lexer;
mod token;

#[macro_use] extern crate lalrpop_util;

//use crate::lexer::Lexer;

fn main() {
  //let lexer = Lexer::new("def f(a) .19 + a");

  // for tok in lexer {
  //   println!("{:?}", tok);
  // }
}

lalrpop_mod!(pub parser); // synthesized by LALRPOP

#[test]
fn parser() {
  let exp = 12.0;
  match parser::ExprParser::new().parse("(1 + 2) * 4") {
    Ok(v) => assert_eq!(exp, v, "Expecting {} but got {}.", exp, v) ,
    Err(err) => panic!(err)
  }
}