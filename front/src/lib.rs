#[macro_use]
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
mod models;
mod pages;
mod workers;

use pages::inbox::Model as InboxModel;
use router::Route;

use yew::prelude::*;
use yew::services::ConsoleService;

use workers::phoenix;
use workers::phoenix::SocketMessage;

pub enum Page {
    Root,
    Inbox,
    Projects,
    Review,
    References,
    Someday,
    Calendar,
    NotFound(String)
}

pub enum ConnectionState {
    Connected,
    Disconnected,
}

pub struct Model {
    page: Page,
    connection: ConnectionState,
    router: Box<Bridge<router::Router<()>>>,
    phoenix: Box<Bridge<phoenix::Phoenix>>,
    console: ConsoleService,
}

pub enum Msg {
    NavigateTo(Page),
    HandleRoute(Route<()>),
    HandlePhoenix(SocketMessage),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(router_callback);

        router.send(router::Request::GetCurrentRoute);

        let phoenix_callback = link.send_back(|socket_message: phoenix::SocketMessage| {
            Msg::HandlePhoenix(socket_message)
        });
        let mut phoenix = phoenix::Phoenix::bridge(phoenix_callback);

        Model {
            page: Page::Inbox,
            connection: ConnectionState::Disconnected,
            router: router,
            phoenix: phoenix,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NavigateTo(page) => {
                let path_segments = match page {
                    Page::Root        => vec!["".into()],
                    Page::Inbox       => vec!["inbox".into()],
                    Page::Projects    => vec!["projects".into()],
                    Page::Review      => vec!["review".into()],
                    Page::References  => vec!["references".into()],
                    Page::Someday     => vec!["someday".into()],
                    Page::Calendar    => vec!["calendar".into()],
                    Page::NotFound(_) => vec!["path_not_found".into()]
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
                self.console.log(&format!("Routing: {}", route.to_route_string()));

                self.page = match route.path_segments.get(0) {
                    Some(first_segment) => {
                        match first_segment.as_str() {
                            ""           => Page::Root,
                            "inbox"      => Page::Inbox,
                            "projects"   => Page::Projects,
                            "review"     => Page::Review,
                            "references" => Page::References,
                            "someday"    => Page::Someday,
                            "calendar"   => Page::Calendar,
                            other        => Page::NotFound(other.into())
                        }
                    },
                    None => Page::NotFound("path_not_found".into())
                };

                true
            }
            Msg::HandlePhoenix(socket) => {
                self.console.log(&format!("Handling socket"));

                false
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::NavigateTo(Page::Inbox),>{ "Go to Inbox" }</button>
                    <button onclick=|_| Msg::NavigateTo(Page::Projects),>{ "Go to Projects" }</button>
                    <button onclick=|_| Msg::NavigateTo(Page::Review),>{ "Go to Review" }</button>
                    <button onclick=|_| Msg::NavigateTo(Page::References),>{ "Go to References" }</button>
                    <button onclick=|_| Msg::NavigateTo(Page::Someday),>{ "Go to Someday" }</button>
                    <button onclick=|_| Msg::NavigateTo(Page::Calendar),>{ "Go to Calendar" }</button>
                </nav>
                <div>
                    {self.page_view()}
                </div>
            </div>
        }
    }
}

impl Model {
    fn page_view(&self) -> Html<Model> {
        match self.page {
            Page::Root | Page::Inbox => html! {
                <>
                {"This corresponds to route 'inbox'"}
                <InboxModel: />
                </>
            },
            Page::Projects => html! {
                <>
                {"This corresponds to route 'projects'"}
                </>
            },
            Page::Review => html! {
                <>
                {"This corresponds to route 'review'"}
                </>
            },
            Page::References => html! {
                <>
                {"This corresponds to route 'references'"}
                </>
            },
            Page::Someday => html! {
                <>
                {"This corresponds to route 'someday'"}
                </>
            },
            Page::Calendar => html! {
                <>
                {"This corresponds to route 'calendar'"}
                </>
            },
            Page::NotFound(ref path) => html! {
                <>
                {format!("Invalid path: '{}'", path)}
                </>
            }
        }
    }
}
