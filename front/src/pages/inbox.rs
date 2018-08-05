use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Model {
    items: Vec<String>,
    adding_item: String,
    console: ConsoleService
}

pub enum Msg {
    Add,
    AddingItemChanged(String),
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
            adding_item: String::new(),
            console: ConsoleService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.console.log(&format!("Adding todo"));

                self.items.push(self.adding_item.clone());
                self.adding_item = String::new();

                true
            }
            Msg::AddingItemChanged(new_adding_item) => {
                self.console.log(&format!("Adding item changed to {}", new_adding_item));
                self.adding_item = new_adding_item;

                true
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

fn view_item(item: &str) -> Html<Model> {
    html! {
        <div>
            { item }
        </div>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <input
                        value=&self.adding_item,
                        oninput=|e| Msg::AddingItemChanged(e.value),/>
                </div>

                <div>
                    <button onclick=|_| Msg::Add,>{ "Add" }</button>
                    <button onclick=|_| Msg::MarkAsTodo,>{ "Mark as todo" }</button>
                    <button onclick=|_| Msg::MarkAsDone,>{ "Mark as done" }</button>
                    <button onclick=|_| Msg::Delete,>{ "Delete" }</button>
                </div>

                { for self.items.iter().map(|item| view_item(item)) }
            </div>
        }
    }
}
