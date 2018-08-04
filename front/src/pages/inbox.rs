use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Model {
    items: Vec<String>,
    console: ConsoleService
}

pub enum Msg {
    Add,
    MarkAsTodo,
    MarkAsDone,
    Delete,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            items: vec![],
            console: ConsoleService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.console.log(&format!("Adding todo"));

                false
            }
            Msg::MarkAsTodo => {
                self.console.log(&format!("Marking item as todo"));

                false
            }
            Msg::MarkAsDone => {
                self.console.log(&format!("Marking item as done"));

                false
            }
            Msg::Delete => {
                self.console.log(&format!("Deleting item"));

                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <button onclick=|_| Msg::Add,>{ "Add" }</button>
                    <button onclick=|_| Msg::MarkAsTodo,>{ "Mark as todo" }</button>
                    <button onclick=|_| Msg::MarkAsDone,>{ "Mark as done" }</button>
                    <button onclick=|_| Msg::Delete,>{ "Delete" }</button>
                </div>
            </div>
        }
    }
}
