use std::rc::Rc;

use crate::koma::{self, KomaKind};
use stylist::css;
use yew::{classes, function_component, html, use_reducer, Callback, Html, Reducible};

const HEIGHT: usize = 9;
const WIDTH: usize = 9;

enum FieldAction {
  Select(isize),
  Move(isize),
}

struct FieldState {
  map: [[MapInfo; WIDTH]; HEIGHT],
  select: Option<usize>,
}

#[derive(Clone, Copy)]
struct MapInfo {
  koma: koma::Koma,
  select: bool,
  movable: bool,
}

impl Default for FieldState {
  fn default() -> Self {
    use koma::{Koma, KomaKind::*};
    let mut map = [[MapInfo {
      koma: Koma::new(None, false),
      select: false,
      movable: false,
    }; 9]; 9];

    map[0][0].koma = Koma::new(Kyo, false);
    map[0][1].koma = Koma::new(Kei, false);
    map[0][2].koma = Koma::new(Silver, false);
    map[0][3].koma = Koma::new(Gold, false);
    map[0][4].koma = Koma::new(King, false);
    map[0][5].koma = Koma::new(Gold, false);
    map[0][6].koma = Koma::new(Silver, false);
    map[0][7].koma = Koma::new(Kei, false);
    map[0][8].koma = Koma::new(Kyo, false);

    map[1][1].koma = Koma::new(Kaku, false);
    map[1][7].koma = Koma::new(Hisha, false);
    for i in 0..WIDTH {
      map[2][i].koma = Koma::new(Fu, false);
    }

    map[8][0].koma = Koma::new(Kyo, true);
    map[8][1].koma = Koma::new(Kei, true);
    map[8][2].koma = Koma::new(Silver, true);
    map[8][3].koma = Koma::new(Gold, true);
    map[8][4].koma = Koma::new(King, true);
    map[8][5].koma = Koma::new(Gold, true);
    map[8][6].koma = Koma::new(Silver, true);
    map[8][7].koma = Koma::new(Kei, true);
    map[8][8].koma = Koma::new(Kyo, true);

    map[7][7].koma = Koma::new(Kaku, true);
    map[7][1].koma = Koma::new(Hisha, true);
    for i in 0..WIDTH {
      map[6][i].koma = Koma::new(Fu, true);
    }

    Self {
      map,
      select: Option::None,
    }
  }
}

impl Reducible for FieldState {
  /// Reducer Action Type
  type Action = FieldAction;

  /// Reducer Function
  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      FieldAction::Select(num) => {
        let mut new_map = self.map.clone();
        for map_row in new_map.iter_mut() {
          for unit in map_row.iter_mut() {
            unit.select = false;
            unit.movable = false;
          }
        }

        let row = num as usize / WIDTH;
        let col = num as usize % WIDTH;

        new_map[row][col].select = true;

        let area = self.map[row][col].koma.can_move();
        let rev = if self.map[row][col].koma.value.0 {
          1
        } else {
          -1
        };
        for move_area in area.iter() {
          match (
            row as isize + move_area.0 * rev,
            col as isize + move_area.1 * rev,
          ) {
            (r, c) if r >= 0 && r < HEIGHT as isize && c >= 0 && c < WIDTH as isize => {
              new_map[r as usize][c as usize].movable = true;
            }
            _ => {}
          }
        }

        Self {
          map: new_map,
          select: Some(num as usize),
        }
        .into()
      }
      FieldAction::Move(num) => {
        let row = num as usize / WIDTH;
        let col = num as usize % WIDTH;
        if let Some(select) = self.select {
          log::info!("move {select}");
          let koma_row = select / WIDTH;
          let koma_col = select % WIDTH;
          let mut new_map = self.map.clone();
          // コマが置かれていないマス or 相手のコマ
          if new_map[row][col].koma.kind == KomaKind::None
            || new_map[koma_row][koma_col].koma.value.0 != new_map[row][col].koma.value.0
          {
            new_map[row][col] = self.map[koma_row][koma_col];
            new_map[koma_row][koma_col] = MapInfo {
              koma: koma::Koma::new(KomaKind::None, new_map[koma_row][koma_col].koma.value.0),
              select: false,
              movable: false,
            }
          }
          for map_row in new_map.iter_mut() {
            for unit in map_row.iter_mut() {
              unit.select = false;
              unit.movable = false;
            }
          }
          return Self {
            map: new_map,
            select: None,
          }
          .into();
        }
        self.into()
      }
    }
  }
}

#[function_component(ShogiComponent)]
pub fn shogi_comp() -> Html {
  let field = use_reducer(FieldState::default);

  html!(
    <div>
      {field
        .map
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
          html!(
            <div class={css!("width: 400px; height: 40px;")}>
              {row.iter().enumerate().map(|(index, col)|{
                let select_koma = {
                  let field = field.clone();
                  if row[index].movable {
                    Callback::from(move |_| {
                      field.dispatch(FieldAction::Move((row_index * WIDTH + index) as isize))
                    })
                  } else {
                    Callback::from(move |_| {
                      field.dispatch(FieldAction::Select((row_index * WIDTH + index) as isize))
                    })
                  }
                };

                let bg = if row[index].select {
                  css!("background-color: aliceblue;")
                }else if row[index].movable {
                  css!("background-color: red;")
                }else{
                  css!("")
                };
                let reverse = if row[index].koma.value.0 {
                  css!("color: blue;")
                }else{
                  css!("transform: rotate(180deg);")
                };
                let style = css!("box-sizing: border-box; display:inline-block; width: 40px; height: 40px; border: solid 0.5px;");
                html!(<div class={classes![style, bg, reverse]} onclick={select_koma}>
                  {format!("{}", col.koma)}
                </div>)
              }).collect::<Html>()}
            </div>
          )
        })
        .collect::<Html>()}
    </div>
  )
}
