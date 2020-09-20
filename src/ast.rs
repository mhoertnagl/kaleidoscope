use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
  pub statements: Vec<Statement>,
}

impl fmt::Display for Module {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.statements.iter().fold(
      String::new(),
      |acc, s| acc + &s.to_string() + "\n")
    )
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
  Let {
    name: String,
    expr: Expr,
  },
  Def {
    name: String,
    params: Vec<String>,
    statements: Vec<Statement>,
  },
  Assign {
    name: String,
    expr: Expr,
  },
  Return {
    expr: Expr,
  }
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Statement::Let{ name, expr } => write!(f, "let {} = {}", name, expr),
      Statement::Def{ name, params, statements } => {
        let ps = params.iter().fold(
          String::new(),
          |acc, s| acc + &s.to_string() + ", "
        );
        let ss = statements.iter().fold(
          String::new(),
          |acc, s| acc + &s.to_string() + "\n"
        );
        write!(f, "def {} ({}) begin {} end", name, ps, ss)
      },
      Statement::Assign{ name, expr } => write!(f, "{} = {}", name, expr),
      Statement::Return{ expr } => write!(f, "return {}", expr),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
  Num(f64),
  BinOp {
    left: Box<Expr>,
    op: Opcode,
    right: Box<Expr>
  },
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Num(n) => write!(f, "{}", n),
      Expr::BinOp { left, op, right } => write!(f, "({} {} {})", left, op, right),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Opcode {
  Mul,
  Div,
  Add,
  Sub,
}

impl fmt::Display for Opcode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Opcode::Mul => write!(f, "{}", "*"),
      Opcode::Div => write!(f, "{}", "/"),
      Opcode::Add => write!(f, "{}", "+"),
      Opcode::Sub => write!(f, "{}", "-"),
    }
  }
}
