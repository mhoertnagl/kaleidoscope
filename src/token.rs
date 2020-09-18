use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  Def,
  End,
  Extern,
  LParens,
  RParens,
  Comma,
  Semicolon,
  Id(String),
  Num(f64),
  Op(String),
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Token::Def => write!(f, "def"),
      Token::End => write!(f, "end"),
      Token::Extern => write!(f, "extern"),
      Token::LParens => write!(f, "("),
      Token::RParens => write!(f, ")"),
      Token::Comma => write!(f, ","),
      Token::Semicolon => write!(f, ";"),
      Token::Id(name) => write!(f, "{}", name),
      Token::Num(val) => write!(f, "{}", val),
      Token::Op(op) => write!(f, "{}", op),
      // tok => write!(f, "{:?}", tok),
    }
  }
}
