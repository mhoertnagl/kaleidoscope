use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

macro_rules! s {
  ($input:expr) => {
    $input.to_string()
  };
}

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
    self.drop_while(|c| c.is_whitespace())
  }

  fn consume_comments(&mut self) {
    while let Some(&c) = self.buf.peek() {
      if c == '#' {
        self.consume_comment()
      } else {
        break
      }
    }
  }

  fn consume_comment(&mut self) {
    if let Some(&c) = self.buf.peek() {
      if c == '#' {
        self.drop_while(|c| c != '\n' && c != '\r')
      }
      self.consume_whitespace()
    }
  }

  fn tokenize_num_or_sym(&mut self, c: char) -> Token {
    if c.is_digit(10) || c == '.' {
      return Token::Num(self.tokenize_num(c))
    }

    let seq = self.tokenize_sym(c);

    match seq.as_str() {
      "def" => Token::Def,
      "end" => Token::End,
      "extern" => Token::Extern,
      _ => Token::Id(seq),
    }
  }

  fn tokenize_num(&mut self, c: char) -> f64 {
    self
      .read_while(c, |d| d.is_digit(10) || d == '.')
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
        break
      }
    }

    sequence
  }

  fn drop_while(&mut self, f: fn(char) -> bool) {
    while let Some(&c) = self.buf.peek() {
      if f(c) {
        self.buf.next();
      } else {
        break
      }
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    self.consume_whitespace();
    self.consume_comments();

    match self.buf.next() {
      Some(c) => Some(match c {
        '(' => Token::LParens,
        ')' => Token::RParens,
        ',' => Token::Comma,
        ';' => Token::Semicolon,
        '+' => Token::Op(s!(c)),
        '-' => Token::Op(s!(c)),
        _ => self.tokenize_num_or_sym(c),
      }),
      None => None,
    }
  }
}

// fn is_sym(c: char) -> bool {
//   c.is_alphanumeric()
// }

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_all() {
    let input = r"
      # Comment 1
      # Comment 2
      def f(a)
        .19 + a  # Comment 3
      end
    ";

    let tokens = vec![
      Token::Def,
      Token::Id(s!("f")),
      Token::LParens,
      Token::Id(s!("a")),
      Token::RParens,
      Token::Num(0.19),
      Token::Op(s!("+")),
      Token::Id(s!("a")),
      Token::End,
    ];

    let mut lexer = Lexer::new(input);

    for exp in tokens {
      if let Some(act) = lexer.next() {
        assert_eq!(exp, act, "Expecting {} but got {}.", exp, act)
      } else {
        panic!("Missing tokens.")
      }
    }
  }
}
