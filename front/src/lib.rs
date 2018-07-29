extern crate stdweb;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate yew;

mod router;
mod routing;
mod dashboard;

use dashboard::DashboardModel;
use router::Route;
use yew::prelude::*;

pub enum Path {
    Dashboard,
    Setting,
    NotFound(String)
}

pub struct Model {
    path: Path,
    router: Box<Bridge<router::Router<()>>>
}

pub enum Msg {
    NavigateTo(Path),
    HandleRoute(Route<()>)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);

        Model {
            path: Path::Dashboard,
            router: router
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NavigateTo(path) => {
                let path_segments = match path {
                    Path::Dashboard => vec!["dashbord".into()],
                    Path::Setting => vec!["setting".into()],
                    Path::NotFound(_) => vec!["path_not_found".into()]
                };

                let route = router::Route {
                    path_segments: path_segments,
                    query: None,
                    fragment: None,
                    state: ()
                };

                self.router.send(router::Request::ChangeRoute(route));

                false
            }
            Msg::HandleRoute(route) => {
                info!("Routing: {}", route.to_route_string());

                self.path = if let Some(first_segment) = route.path_segments.get(0) {
                    match first_segment.as_str() {
                        "dashbord" => Path::Dashboard,
                        "setting" => Path::Setting,
                        other => Path::NotFound(other.into())
                    }
                } else {
                    Path::NotFound("path_not_found".into())
                };

                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::NavigateTo(Path::Dashboard),>{ "Go to Dashboard" }</button>
                    <button onclick=|_| Msg::NavigateTo(Path::Setting),>{ "Go to Setting" }</button>
                </nav>
                <div>
                    {self.path.view()}
                </div>
            </div>
        }
    }
}

impl Renderable<Model> for Path {
    fn view(&self) -> Html<Model> {
        match *self {
            Path::Dashboard => html! {
                <>
                {"This corresponds to route 'dashboard'"}
                <DashboardModel: />
                </>
            },
            Path::Setting => html! {
                <>
                {"This corresponds to route 'setting'"}
                </>
            },
            Path::NotFound(ref path) => html! {
                <>
                {format!("Invalid path: '{}'", path)}
                </>
            }
        }
    }
}
