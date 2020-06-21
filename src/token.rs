#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  Def,            // The 'def' keyword.
  Extern,         // The 'extern' keyword.
  LParens,        // Left opening parenthesis '('.
  RParens,        // Right closing parenthesis ')'.
  Comma,          // The comma ','.
  Semicolon,      // The semicolon ';'.
  Id(String),     // An identifer.
  Num(f64),       // A floating-point number.
  Op(String),     // an operator.
}