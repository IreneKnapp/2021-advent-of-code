use std::fmt;


#[derive(Clone)]
pub enum Value {
  Literal(i64),
  Pair(Box<Pair>),
}

#[derive(Clone)]
pub struct Pair {
  pub left: Value,
  pub right: Value,
}


impl fmt::Debug for Value {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Literal(n) => { write!(formatter, "{}", n) }
      Value::Pair(pair) => { write!(formatter, "{:?}", pair) }
    }
  }
}


impl fmt::Debug for Pair {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "[{:?},{:?}]", self.left, self.right)
  }
}

