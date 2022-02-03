use std::fmt;

#[derive(Debug)]
pub enum List {
  Cons(i32, Box<List>),
  Nil,
}

impl List {
  pub fn new() -> Self {
      Self::Nil
  }
  pub fn cons(elem: i32, ls: Self) -> Self {
      Self::Cons(elem, Box::new(ls))
  }
  pub fn uncons(ls: Self) -> Option<(i32, Self)> {
      match ls {
          List::Cons(head, tail) => Some((head, *tail)),
          List::Nil => None,
      }
  }
  pub fn from_collection(col: &[i32]) -> Self {
      col.iter()
          .rev()
          .fold(Self::new(), |ls, elem| Self::cons(*elem, ls))
  }
  pub fn len(&self) -> usize {
      match self {
          List::Nil => 0,
          List::Cons(_, tail) => 1 + tail.len(),
      }
  }
}

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
          List::Nil => write!(f, "Nil"),
          List::Cons(head, tail) => write!(f, "{}->", head).and_then(|_| tail.fmt(f)),
      }
  }
}

#[derive(Debug)]
pub struct MyObj {
  age: usize,
  name: String,
}

impl Default for MyObj {
  fn default() -> Self {
      Self {
          age: 0,
          name: "John Doe".to_string(),
      }
  }
}

pub fn get_nums<A, B>(count: usize) -> A
where
  A: FromIterator<B>,
  B: Default,
{
  (0..count).map(|_| B::default()).collect()
}
