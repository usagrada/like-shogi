use crate::koma;
use std::rc::Rc;
use stylist::{css, yew::styled_component};
use yew::{function_component, html, use_reducer, Html, Properties, Reducible};

const HEIGHT: usize = 9;
const WIDTH: usize = 9;

enum FieldAction {}

// #[derive(Clone)]
struct FieldState {
  map: [[koma::Koma; WIDTH]; HEIGHT],
}

impl Default for FieldState {
  fn default() -> Self {
    use koma::Koma;
    let mut map = [[Koma::None; 9]; 9];
    // map[0][0] = Some(Koma::Kyo(false, false));
    // map[0][1] = Some(Koma::Kei(false, false));
    // map[0][2] = Some(Koma::Silver(false, false));
    // map[0][3] = Some(Koma::Gold(false));
    // map[0][4] = Some(Koma::King(false));
    // map[0][5] = Some(Koma::Gold(false));
    // map[0][6] = Some(Koma::Silver(false, false));
    // map[0][7] = Some(Koma::Kei(false, false));
    // map[0][8] = Some(Koma::Kyo(false, false));

    // map[1][1] = Some(Koma::Kaku(false, false));
    // map[1][7] = Some(Koma::Hisha(false, false));
    // for i in 0..WIDTH {
    //   map[2][i] = Some(Koma::Fu(false, false))
    // }

    // map[8][0] = Some(Koma::Kyo(false, false));
    // map[8][1] = Some(Koma::Kei(false, false));
    // map[8][2] = Some(Koma::Silver(false, false));
    // map[8][3] = Some(Koma::Gold(false));
    // map[8][4] = Some(Koma::King(false));
    // map[8][5] = Some(Koma::Gold(false));
    // map[8][6] = Some(Koma::Silver(false, false));
    // map[8][7] = Some(Koma::Kei(false, false));
    // map[8][8] = Some(Koma::Kyo(false, false));

    // map[7][7] = Some(Koma::Kaku(false, false));
    // map[7][1] = Some(Koma::Hisha(false, false));
    // for i in 0..WIDTH {
    //   map[6][i] = Some(Koma::Fu(false, false))
    // }

    map[0][0] = Koma::Kyo(false, false);
    map[0][1] = Koma::Kei(false, false);
    map[0][2] = Koma::Silver(false, false);
    map[0][3] = Koma::Gold(false);
    map[0][4] = Koma::King(false);
    map[0][5] = Koma::Gold(false);
    map[0][6] = Koma::Silver(false, false);
    map[0][7] = Koma::Kei(false, false);
    map[0][8] = Koma::Kyo(false, false);

    map[1][1] = Koma::Kaku(false, false);
    map[1][7] = Koma::Hisha(false, false);
    for i in 0..WIDTH {
      map[2][i] = Koma::Fu(false, false)
    }

    map[8][0] = Koma::Kyo(false, false);
    map[8][1] = Koma::Kei(false, false);
    map[8][2] = Koma::Silver(false, false);
    map[8][3] = Koma::Gold(false);
    map[8][4] = Koma::King(false);
    map[8][5] = Koma::Gold(false);
    map[8][6] = Koma::Silver(false, false);
    map[8][7] = Koma::Kei(false, false);
    map[8][8] = Koma::Kyo(false, false);

    map[7][7] = Koma::Kaku(false, false);
    map[7][1] = Koma::Hisha(false, false);
    for i in 0..WIDTH {
      map[6][i] = Koma::Fu(false, false)
    }

    Self { map }
  }
}

impl Reducible for FieldState {
  /// Reducer Action Type
  type Action = FieldAction;

  /// Reducer Function
  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    self.into()
  }
}

enum KomaAction {}

struct KomaState {}

#[function_component(ShogiComponent)]
pub fn shogi_comp() -> Html {
  let field = use_reducer(FieldState::default);

  html!(
    <div>
      {field
        .map
        .iter()
        .map(|row| {
          html! (<ShogiRow row={*row} />)
        })
        .collect::<Html>()}
    </div>
  )
}

#[derive(Properties, PartialEq)]
pub struct ShogiRowProps {
  row: [koma::Koma; WIDTH],
}

#[styled_component(ShogiRow)]
fn shogi_row(prop: &ShogiRowProps) -> Html {
  html!(
    <div class={css!("width: 400px; height: 40px;")}>
      {prop.row.iter().map(|col|{
        html!(<div class={css!("box-sizing: border-box; display:inline-block; width: 40px; height: 40px; border: solid 0.5px;")}>
          {format!("{}", col)}
        </div>)
      }).collect::<Html>()}
    </div>
  )
}
