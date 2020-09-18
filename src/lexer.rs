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
    self.drop_while(char::is_whitespace)
  }

  fn consume_comments(&mut self) {
    while let Some('#') = self.buf.peek() {
      self.drop_while(is_not_newline);
      self.consume_whitespace()
    }
  }

  fn tokenize_num_or_sym(&mut self, c: char) -> Token {
    if is_digit_or_dot(c) {
      return Token::Num(self.tokenize_num(c))
    }
    match self.tokenize_sym(c).as_str() {
      "def" => Token::Def,
      "end" => Token::End,
      "extern" => Token::Extern,
      seq => Token::Id(s!(seq)),
    }
  }

  fn tokenize_num(&mut self, c: char) -> f64 {
    self.read_while(c, is_digit_or_dot).parse().unwrap()
  }

  fn tokenize_sym(&mut self, c: char) -> String {
    self.read_while(c, char::is_alphanumeric)
  }

  fn read_while(&mut self, c: char, f: fn(char) -> bool) -> String {
    // let s = self.buf.take_while(|&c| f(c)).fold(String::new(), |mut s, c| { s.push(c); s });
    let mut sequence = s!(c);
    while let Some(&d) = self.buf.peek().filter(|&&c| f(c)) {
      sequence.push(d);
      self.buf.next();
    }
    sequence
  }

  fn drop_while(&mut self, f: fn(char) -> bool) {
    while self.buf.peek().filter(|&&c| f(c)).is_some() {
      self.buf.next();
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    self.consume_whitespace();
    self.consume_comments();
    self.buf.next().map(|c| match c {
      '(' => Token::LParens,
      ')' => Token::RParens,
      ',' => Token::Comma,
      ';' => Token::Semicolon,
      '+' => Token::Op(s!(c)),
      '-' => Token::Op(s!(c)),
      _ => self.tokenize_num_or_sym(c),
    })
  }
}

fn is_not_newline(c: char) -> bool {
  is_newline(c) == false
}

fn is_newline(c: char) -> bool {
  c == '\r' || c == '\n'
}

fn is_digit_or_dot(c: char) -> bool {
  c.is_digit(10) || c == '.'
}

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
