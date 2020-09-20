mod lexer;
mod token;
mod ast;

#[macro_use] extern crate lalrpop_util;

fn main() {
  match parser::ModuleParser::new().parse("let a = 0") {
    Ok(v) => println!("{}", v),
    Err(err) => println!("{}", err),
  }
}

lalrpop_mod!(pub parser); // synthesized by LALRPOP

#[test]
fn parser() {
  let exp = 12.0;
  match parser::ModuleParser::new().parse("(1 + 2) * 4") {
    Ok(v) => assert_eq!(exp, v, "Expecting {} but got {}.", exp, v) ,
    Err(err) => panic!(err),
  }
}