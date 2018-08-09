use yew::prelude::*;
use yew::services::ConsoleService;

use todo::Todo;

pub struct Model {
    todos: Vec<Todo>,
    adding_todo: String,
    console: ConsoleService
}

pub enum Msg {
    Add,
    AddingTodoChanged(String),
    Delete(usize, String),
    Nope,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            todos: vec![],
            adding_todo: String::new(),
            console: ConsoleService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.console.log(&format!("Adding todo"));

                let new_todo = self.build_todo();
                self.todos.push(new_todo);

                true
            }
            Msg::AddingTodoChanged(new_adding_todo) => {
                self.console.log(&format!("Adding item changed to {}", new_adding_todo));
                self.adding_todo = new_adding_todo;

                true
            }
            Msg::Delete(index, uuid) => {
                self.console.log(&format!("Deleting todo: {} {}", index, uuid));
                let _removing_todo = self.todos.remove(index);

                true
            }
            Msg::Nope => { false }
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
                    <input
                        value=&self.adding_todo,
                        oninput=|e| Msg::AddingTodoChanged(e.value),
                        onkeypress=|e| {
                            if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                        } ,/>
                </div>

                <div>
                    <button onclick=|_| Msg::Add,>{ "Add" }</button>
                </div>

                { for self.todos.iter().enumerate().map(|(i, todo)| self.view_todo(i, todo.clone())) }
            </div>
        }
    }
}

impl Model {
    fn view_todo(&self, index: usize, todo: Todo) -> Html<Model> {
        html! {
            <div>
                { &todo.content }
                <button onclick=|_| Msg::Delete(index, todo.uuid.clone()),>{ "Delete" }</button>
            </div>
        }
    }


    fn build_todo(&mut self) -> Todo {
        let adding_todo_content = self.adding_todo.clone();
        self.adding_todo = String::new();

        Todo {
            uuid: String::new(),
            content: adding_todo_content,
            context: String::new(),
            project: String::new(),
        }
    }
}
