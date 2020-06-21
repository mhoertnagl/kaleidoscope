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

    // self.read_while(' ', |d| d.is_whitespace());
  }

  fn tokenize_num_or_sym(&mut self, c: char) -> Token {
    if c.is_digit(10) {
      Token::Num(self.tokenize_num(c))
    } else {
      Token::Id(self.tokenize_sym(c))
    }
  }

  fn tokenize_num(&mut self, c: char) -> f64 {
    self.read_while(c, |d| d.is_digit(10))
        .parse()
        .unwrap()

    // let mut s = String::new();
    // s.push(c);

    // while let Some(d) = self.buf.peek() {
    //   if d.is_digit(10) {
    //     s.push(self.buf.next().unwrap());
    //   } else {
    //     break;
    //   }
    // }

    // s.parse().unwrap()
  }

  fn tokenize_sym(&mut self, c: char) -> String {
    self.read_while(c, |d| !(d.is_digit(10) || d.is_whitespace()))
  }

  fn read_while(&mut self, c: char, f: fn(char) -> bool) -> String {
    let mut s = String::new();
    s.push(c);

    while let Some(&d) = self.buf.peek() {
      if f(d) {
        s.push(self.buf.next().unwrap());
      } else {
        break;
      }
    }

    s
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
        // '0'..='9' => Some(Token::Num(self.tokenize_num(c))),
         x  => Some(self.tokenize_num_or_sym(x))
        //  x  => if x.is_digit(10) {
        //   Some(Token::Num(self.tokenize_num(x)))
        //  } else {
        //   Some(Token::Id(self.tokenize_sym(x)))
        //  },
      },
      None => None,
    }
  }
}