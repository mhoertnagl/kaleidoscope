mod token;
mod lexer;

use crate::lexer::Lexer;

fn main() {
  let lexer = Lexer::new("(19 + 23)");

  for tok in lexer {
    println!("{:?}", tok);
  }
}
