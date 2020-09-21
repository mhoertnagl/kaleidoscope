mod lexer;
mod token;
mod ast;

#[macro_use] extern crate lalrpop_util;

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
  let expr = parser::ModuleParser::new()
    .parse("(1 + 2) * 4")
    .unwrap();

  assert_eq!(&format!("{:?}", expr), "((1 + 2) * 4)");
}