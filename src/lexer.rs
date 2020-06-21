use std::str::Chars;
use std::iter::Peekable;
use crate::token::Token;

pub struct Lexer<'a> {
  buf: Peekable<Chars<'a>>,
  // row: u32,
  // col: u32,
}

impl<'a> Lexer<'a> {

  pub fn new(input: &'a str) -> Lexer {
    Lexer {
      buf: input.chars().peekable(),
      // row: 1,
      // col: 1,
    }
  }

  // fn next_char(&mut self) -> Option<char> {
  //   match self.buf.next() {
  //     Some(c) => {
  //       if c == '\n' {
  //         self.row += 1;
  //         self.col = 1;
  //       } else {
  //         self.col += 1;
  //       }
  //       Some(c)
  //     },
  //     None => None
  //   }
  // }

  fn consume_whitespace(&mut self) {
    while let Some(c) = self.buf.peek() {
      if c.is_whitespace() {
        self.buf.next();
      } else {
        break;
      }
    }
  }

  fn tokenize_num_or_sym(&mut self, c: char) -> Token {
    if c.is_digit(10) {
      return Token::Num(self.tokenize_num(c));
    }

    let seq = self.tokenize_sym(c);

    match seq.as_str() {
      "let"    => Token::Def,
      "extern" => Token::Extern,
       _       => Token::Id(seq),
    }
  }

  fn tokenize_num(&mut self, c: char) -> f64 {
    self.read_while(c, |d| d.is_digit(10))
        .parse()
        .unwrap()
  }

  fn tokenize_sym(&mut self, c: char) -> String {
    self.read_while(c, |d| d.is_alphanumeric())
  }

  fn read_while(&mut self, c: char, f: fn(char) -> bool) -> String {
    let mut sequence = String::new();
    sequence.push(c);

    while let Some(&d) = self.buf.peek() {
      if f(d) {
        sequence.push(d);
        self.buf.next();
      } else {
        break;
      }
    }

    sequence
  }

  fn op_token(&mut self, c: char) -> Token {
    Token::Op(c.to_string())
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {

    self.consume_whitespace();

    match self.buf.next() {
      Some(c) => match c {
        '(' => Some(Token::LParens),
        ')' => Some(Token::RParens),
        ',' => Some(Token::Comma),
        ';' => Some(Token::Semicolon),
        '+' => Some(self.op_token(c)),
        '-' => Some(self.op_token(c)),
         _  => Some(self.tokenize_num_or_sym(c)),
      },
      None => None,
    }
  }
}

fn is_sym(c: char) -> bool {
  c.is_alphanumeric()
}
