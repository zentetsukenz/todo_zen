use yew::prelude::*;
use yew::services::{ConsoleService};
use yew::format::Json;

use models::next_action::NextAction;
use models::anything::Anything;

pub struct Model {
    new_item: String,
    inbox: Vec<Anything>,
    actionable: Vec<Actionable>,
    next_actions: Vec<NextAction>,
    console: ConsoleService,
}

struct Actionable {
    anything: Anything,
    next_action: String,
    project: Option<String>,
    context: Option<String>
}

pub enum Msg {
    AddNewItem,
    NewItemChanged(String),
    DeleteItemFromInbox(usize),
    MarkAnythingAsActionable(usize),
    NextActionChanged(usize, String),
    ProjectChanged(usize, String),
    ContextChanged(usize, String),
    SetNextAction(usize),
    Nope,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            new_item: String::new(),
            inbox: vec![],
            actionable: vec![],
            next_actions: vec![],
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddNewItem => {
                self.console.log(&format!("AddNewItem"));

                let new_anything = Anything::new(self.new_item.clone());
                self.new_item = String::new();
                self.inbox.push(new_anything);

                true
            }
            Msg::NewItemChanged(to) => {
                self.console.log(&format!("NewItemChanged to {}", to));
                self.new_item = to;

                true
            }
            Msg::DeleteItemFromInbox(index) => {
                self.console.log(&format!("Deleting todo: {}", index));
                let _removing_todo = self.inbox.remove(index);

                true
            }
            Msg::MarkAnythingAsActionable(index) => {
                self.console.log(&format!("Mark as actionable for todo: {}", index));
                let anything = self.inbox.remove(index);
                let actionable_anything = Actionable {
                    anything: anything,
                    next_action: String::new(),
                    project: None,
                    context: None,
                };
                self.actionable.push(actionable_anything);

                true
            }
            Msg::NextActionChanged(index, to) => {
                self.console.log(&format!("NextActionChanged to: {}", to));
                self.actionable[index].next_action = to;

                true
            }
            Msg::ProjectChanged(index, to) => {
                self.console.log(&format!("ProjectChanged to: {}", to));
                self.actionable[index].project = Some(to);

                true
            }
            Msg::ContextChanged(index, to) => {
                self.console.log(&format!("ContextChanged to: {}", to));
                self.actionable[index].context = Some(to);

                true
            }
            Msg::SetNextAction(index) => {
                self.console.log(&format!("Setting next action for todo: {}", index));

                let actionable = self.actionable.remove(index);
                let new_next_action = NextAction::new(actionable.next_action, actionable.project, actionable.context);
                self.next_actions.push(new_next_action);

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
                { self.view_new_item() }
                { self.view_inbox() }
                { self.view_actionable() }
                { self.view_next_actions() }
            </div>
        }
    }
}

impl Model {
    fn view_new_item(&self) -> Html<Model> {
        html! {
            <div>
                <div>
                    <input
                        value=&self.new_item,
                        oninput=|e| Msg::NewItemChanged(e.value),
                        onkeypress=|e| {
                            if e.key() == "Enter" { Msg::AddNewItem } else { Msg::Nope }
                        } ,/>
                </div>
                <div>
                    <button onclick=|_| Msg::AddNewItem,>{ "AddNewItem" }</button>
                </div>
            </div>
        }
    }

    fn view_inbox(&self) -> Html<Model> {
        html! {
            <div>
                <h1>{ "Inbox items" }</h1>
                <table>
                    <tr>
                        <th>{ "Content" }</th>
                        <th></th>
                    </tr>
                    { for self.inbox.iter().enumerate().map(|(i, anything)| self.view_anything(i, anything.clone())) }
                </table>
            </div>
        }
    }

    fn view_actionable(&self) -> Html<Model> {
        html! {
            <div>
                <h1>{ "Actionables" }</h1>
                <table>
                    <tr>
                        <th>{ "Content" }</th>
                        <th>{ "Next Action" }</th>
                        <th>{ "Project" }</th>
                        <th>{ "Context" }</th>
                        <th></th>
                    </tr>
                    { for self.actionable.iter().enumerate().map(|(i, anything)| self.view_actionable_anything(i, anything.clone())) }
                </table>
            </div>
        }
    }

    fn view_next_actions(&self) -> Html<Model> {
        html! {
            <div>
                <h1>{ "Next Actions" }</h1>
                <table>
                    <tr>
                        <th>{ "Next Action" }</th>
                        <th>{ "Project" }</th>
                        <th>{ "Context" }</th>
                        <th></th>
                    </tr>
                    { for self.next_actions.iter().enumerate().map(|(i, next_action)| self.view_next_action(i, next_action.clone())) }
                </table>
            </div>
        }
    }

    fn view_anything(&self, index: usize, anything: Anything) -> Html<Model> {
        html! {
            <tr>
                <td>
                    { &anything.content }
                </td>
                <td>
                    <button onclick=|_| Msg::MarkAnythingAsActionable(index),>{ "Actionable" }</button>
                    <button onclick=|_| Msg::DeleteItemFromInbox(index),>{ "Won't do" }</button>
                </td>
            </tr>
        }
    }

    fn view_actionable_anything(&self, index: usize, actionable: &Actionable) -> Html<Model> {
        let project = match &actionable.project {
            Some(project) => project.clone(),
            None => String::new()
        };

        let context = match &actionable.context {
            Some(context) => context.clone(),
            None => String::new()
        };

        html! {
            <tr>
                <td>
                    { &actionable.anything.content }
                </td>
                <td>
                    <input
                        value=&actionable.next_action,
                        oninput=|e| Msg::NextActionChanged(index, e.value),
                        onkeypress=|e| {
                            if e.key() == "Enter" { Msg::SetNextAction(index) } else { Msg::Nope }
                        }, />
                </td>
                <td>
                    <input
                        value=project,
                        oninput=|e| Msg::ProjectChanged(index, e.value),
                        onkeypress=|e| {
                            if e.key() == "Enter" { Msg::SetNextAction(index) } else { Msg::Nope }
                        }, />
                </td>
                <td>
                    <input
                        value=context,
                        oninput=|e| Msg::ContextChanged(index, e.value),
                        onkeypress=|e| {
                            if e.key() == "Enter" { Msg::SetNextAction(index) } else { Msg::Nope }
                        }, />
                </td>
                <td>
                    <button onclick=|_| Msg::SetNextAction(index),>{ "Set Next Action" }</button>
                </td>
            </tr>
        }
    }

    fn view_next_action(&self, _index: usize, next_action: &NextAction) -> Html<Model> {
        let project = match &next_action.project {
            Some(project) => project.clone(),
            None => String::new(),
        };

        let context = match &next_action.context {
            Some(context) => context.clone(),
            None => String::new(),
        };

        html!{
            <tr>
                <td>
                    { &next_action.content }
                </td>
                <td>
                    { project }
                </td>
                <td>
                    { context }
                </td>
                <td>
                </td>
            </tr>
        }
    }
}
