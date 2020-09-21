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
    expr: Box<Expr>,
  },
  Def {
    name: String,
    params: Vec<String>,
    statements: Vec<Statement>,
  },
  Assign {
    name: String,
    expr: Box<Expr>,
  },
  Expr {
    expr: Box<Expr>,
  },
  Return {
    expr: Box<Expr>,
  }
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Statement::Let{ name, expr } => write!(f, "let {} = {};", name, expr),
      Statement::Def{ name, params, statements } => {
        let ps = params.iter().fold(
          String::new(),
          |acc, s| acc + &s.to_string() + ", "
        );
        let ss = statements.iter().fold(
          String::new(),
          |acc, s| acc + &s.to_string() + "\n"
        );
        write!(f, "def {} ({}) begin \n{}end", name, ps, ss)
      },
      Statement::Assign{ name, expr } => write!(f, "{} = {};", name, expr),
      Statement::Expr{ expr } => write!(f, "{};", expr),
      Statement::Return{ expr } => write!(f, "return {};", expr),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
  Atom(Atom),
  FunCall {
    name: String,
    args: Vec<Box<Expr>>,
  },
  BinOp {
    left: Box<Expr>,
    op: Opcode,
    right: Box<Expr>
  },
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Atom(a) => write!(f, "{}", a),
      Expr::FunCall{ name, args } => {
        let ps = args.iter().fold(
          String::new(),
          |acc, s| acc + &s.to_string() + ", "
        );
        write!(f, "{}({})", name, ps)
      },
      Expr::BinOp { left, op, right } => write!(f, "({} {} {})", left, op, right),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
  Num(f64),
  Id(String),
}

impl fmt::Display for Atom {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Atom::Num(n) => write!(f, "{}", n),
      Atom::Id(id) => write!(f, "{}", id),
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
