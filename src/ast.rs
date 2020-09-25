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
  Let(LetStatement),
  Def(DefStatement),
  Assign(AssignStatement),
  Expr(ExprStatement),
  Return(ReturnStatement),
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Statement::Let(e) => e.fmt(f),
      Statement::Def(e) => e.fmt(f),
      Statement::Assign(e) => e.fmt(f),
      Statement::Expr(e) => e.fmt(f),
      Statement::Return(e) => e.fmt(f),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LetStatement {
  pub name: String,
  pub expr: Box<Expr>,
}

impl LetStatement {
  pub fn new(name: String, expr: Box<Expr>) -> LetStatement {
    LetStatement {
      name: name,
      expr: expr,
    }
  }
}

impl fmt::Display for LetStatement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "let {} = {};", self.name, self.expr)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefStatement {
  name: String,
  params: Vec<String>,
  statements: Vec<Statement>,
}

impl DefStatement {
  pub fn new(
    name: String,
    params: Option<Vec<String>>,
    statements: Vec<Statement>) -> DefStatement {
    DefStatement {
      name: name,
      params: params.unwrap_or(vec![]),
      statements: statements,
    }
  }
}

impl fmt::Display for DefStatement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let ps = self.params.join(", ");
    let ss = self.statements.iter().fold(
      String::new(),
      |acc, s| acc + &s.to_string() + "\n"
    );
    write!(f, "def {} ({}) begin \n{}end", self.name, ps, ss)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignStatement {
  pub name: String,
  pub expr: Box<Expr>,
}

impl AssignStatement {
  pub fn new(name: String, expr: Box<Expr>) -> AssignStatement {
    AssignStatement {
      name: name,
      expr: expr,
    }
  }
}

impl fmt::Display for AssignStatement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} = {};", self.name, self.expr)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStatement {
  pub expr: Box<Expr>,
}

impl ExprStatement {
  pub fn new(expr: Box<Expr>) -> ExprStatement {
    ExprStatement {
      expr: expr,
    }
  }
}

impl fmt::Display for ExprStatement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{};", self.expr)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
  pub expr: Box<Expr>,
}

impl ReturnStatement {
  pub fn new(expr: Box<Expr>) -> ReturnStatement {
    ReturnStatement {
      expr: expr,
    }
  }
}

impl fmt::Display for ReturnStatement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "return {};", self.expr)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
  Atom(Atom),
  FunCall(FunCallExpr),
  BinOp(BinOpExpr),
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Atom(e) => e.fmt(f),
      Expr::FunCall(e) => e.fmt(f),
      Expr::BinOp(e) => e.fmt(f),
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
      Atom::Num(n) => n.fmt(f),
      Atom::Id(id) => id.fmt(f),
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
      Opcode::Mul => "*".fmt(f),
      Opcode::Div => "/".fmt(f),
      Opcode::Add => "+".fmt(f),
      Opcode::Sub => "-".fmt(f),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunCallExpr {
  pub name: String,
  pub args: Vec<Box<Expr>>,
}

impl FunCallExpr {
  pub fn new(name: String, args: Option<Vec<Box<Expr>>>) -> FunCallExpr {
    FunCallExpr {
      name: name,
      args: args.unwrap_or(vec![]),
    }
  }
}

impl fmt::Display for FunCallExpr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let px: Vec<String> = self.args.iter().map(|a| a.to_string()).collect();
    let ps = px.join(", ");
    write!(f, "{}({})", self.name, ps)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinOpExpr {
  pub left: Box<Expr>,
  pub op: Opcode,
  pub right: Box<Expr>
}

impl BinOpExpr {
  pub fn new(left: Box<Expr>, op: Opcode, right: Box<Expr>) -> BinOpExpr {
    BinOpExpr {
      left: left,
      op: op,
      right: right,
    }
  }
}

impl fmt::Display for BinOpExpr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({} {} {})", self.left, self.op, self.right)
  }
}
