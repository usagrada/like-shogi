use std::{
  fmt::{self, Display},
  vec,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct KomaNone;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct King(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Gold(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Silver(pub bool, pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Kei(pub bool, pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Kyo(pub bool, pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hisha(pub bool, pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Kaku(pub bool, pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fu(pub bool, pub bool);

impl King {
  fn can_move() -> Vec<(isize, isize)> {
    vec![
      (-1, -1),
      (-1, 0),
      (-1, 1),
      (0, -1),
      (0, 1),
      (1, -1),
      (1, 0),
      (1, 1),
    ]
  }
}

impl Gold {
  fn can_move() -> Vec<(isize, isize)> {
    vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0)]
  }
}

impl Silver {
  fn can_move() -> Vec<(isize, isize)> {
    vec![(-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 1)]
  }
}

impl Kyo {
  fn can_move() -> Vec<(isize, isize)> {
    let mut _can = vec![];
    for i in 1..=9 {
      _can.push((-i, 0));
    }
    _can
  }
}

impl Kei {
  pub fn can_move() -> Vec<(isize, isize)> {
    vec![(-2, -1), (-2, 1)]
  }
}

impl Fu {
  fn can_move() -> Vec<(isize, isize)> {
    vec![(-1, 0)]
  }
}

impl Kaku {
  fn can_move() -> Vec<(isize, isize)> {
    let mut _can = vec![];
    for i in 1..=9 {
      _can.push((i, i));
      _can.push((i, -i));
      _can.push((-i, i));
      _can.push((-i, -i));
    }
    _can
  }
}

impl Hisha {
  fn can_move() -> Vec<(isize, isize)> {
    let mut _can = vec![(0, 0)];
    for i in 1..9 {
      _can.push((-i, 0));
      _can.push((i, 0));
      _can.push((0, -i));
      _can.push((0, i));
    }
    _can
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KomaKind {
  None,
  King,
  Gold,
  Silver,
  Kei,
  Kyo,
  Hisha,
  Kaku,
  Fu,
}

type KomaValue = (bool, bool);

#[derive(Clone, Copy)]
pub struct Koma {
  pub kind: KomaKind,
  pub value: KomaValue,
}

impl Koma {
  pub fn new(kind: KomaKind, is_me: bool) -> Self {
    Self {
      kind,
      value: (is_me, false),
    }
  }
  pub fn can_move(&self) -> Vec<(isize, isize)> {
    match self.kind {
      KomaKind::King => King::can_move(),
      KomaKind::Gold => Gold::can_move(),
      KomaKind::Silver => Silver::can_move(),
      KomaKind::Hisha => Hisha::can_move(),
      KomaKind::Kaku => Kaku::can_move(),
      KomaKind::Fu => Fu::can_move(),
      KomaKind::Kei => Kei::can_move(),
      KomaKind::Kyo => Kyo::can_move(),
      _ => vec![],
    }
  }
}

impl Display for Koma {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use KomaKind::*;
    match self.kind {
      Fu => write!(f, "歩"),
      Gold => write!(f, "金"),
      Hisha => write!(f, "飛"),
      Kaku => write!(f, "角"),
      Kyo => write!(f, "香"),
      King => write!(f, "王"),
      Silver => write!(f, "銀"),
      Kei => write!(f, "桂"),
      _ => write!(f, "　"),
    }
  }
}
