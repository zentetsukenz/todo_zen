use router;
use router::Route;
use yew::prelude::*;

pub struct DashboardModel {
    todos: Vec<String>,
    router: Box<Bridge<router::Router<()>>>
}

pub enum Msg {
    Navigate(Vec<Msg>),
    Add,
    MarkAsDone,
    Delete,
    HandleRoute(Route<()>)
}

impl Component for DashboardModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);

        DashboardModel {
            todos: vec![],
            router: router
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Navigate(msgs) => {
                for msg in msgs {
                    self.update(msg);
                }

                let mut path_segments = vec!["dashboard".into()];

                let route = router::Route {
                    path_segments: path_segments,
                    query: None,
                    fragment: None,
                    state: (),
                };

                self.router.send(router::Request::ChangeRoute(route));

                false
            }
            Msg::HandleRoute(route) => {
                info!("Routing: {}", route.to_route_string());

                false
            }
            Msg::Add => {
                info!("Adding todo");

                false
            }
            Msg::MarkAsDone => {
                info!("Marking todo as done");

                false
            }
            Msg::Delete => {
                info!("Deleting todo");

                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<DashboardModel> for DashboardModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <button onclick=|_| Msg::Add,>{ "Add" }</button>
                    <button onclick=|_| Msg::MarkAsDone,>{ "Mark as done" }</button>
                    <button onclick=|_| Msg::Delete,>{ "Delete" }</button>
                </div>
            </div>
        }
    }
}
