extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {}
pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                { "Hello, world!" }
            </div>
        }
    }
}
