mod lexer;
mod token;

use crate::lexer::Lexer;

fn main() {
    let lexer = Lexer::new("def f(a) .19 + a");

    for tok in lexer {
        println!("{:?}", tok);
    }
}
