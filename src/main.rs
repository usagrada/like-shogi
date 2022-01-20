#![feature(in_band_lifetimes)]

use yew::prelude::*;
mod component;
mod koma;

struct App;

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <main>
        <component::ShogiComponent />
      </main>
    }
  }
}

fn main() {
  yew::start_app::<App>();
}
