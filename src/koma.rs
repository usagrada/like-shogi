use std::fmt::{self, Display};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Koma {
  None,
  King(bool),
  Gold(bool),
  Silver(bool, bool),
  Kei(bool, bool),
  Kyo(bool, bool),
  Hisha(bool, bool),
  Kaku(bool, bool),
  Fu(bool, bool),
}

impl Display for Koma {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Self::Fu(_, _) => write!(f, "歩"),
      Self::Gold(_) => write!(f, "金"),
      Self::Hisha(_, _) => write!(f, "飛"),
      Self::Kaku(_, _) => write!(f, "角"),
      Self::Kyo(_, _) => write!(f, "香"),
      Self::King(_) => write!(f, "王"),
      Self::Silver(_, _) => write!(f, "銀"),
      Self::Kei(_, _) => write!(f, "桂"),
      _ => write!(f, "　"),
    }
  }
}

struct King(bool);
struct Gold(bool);
struct Silver(bool, bool);
struct Kei(bool, bool);
struct Kyo(bool, bool);
struct Hisha(bool, bool);
struct Kaku(bool, bool);
struct Fu(bool, bool);

impl Display for King {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "王")
  }
}

impl Display for Gold {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "金")
  }
}
impl Display for Silver {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "金")
  }
}
impl Display for Kei {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "桂")
  }
}
impl Display for Kyo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "香")
  }
}
impl Display for Hisha {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "飛")
  }
}
impl Display for Kaku {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "角")
  }
}
impl Display for Fu {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "歩")
  }
}
