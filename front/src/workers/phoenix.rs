use yew::prelude::worker::*;

use std::collections::HashSet;

use stdweb::Value;

use serde::Serialize;
use serde::Deserialize;

pub enum Msg {
    OnError,
    OnClose,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Join(String),
}

impl Transferable for Request {}

#[derive(Serialize, Deserialize)]
pub struct Socket {}

impl Transferable for Socket {}

pub struct Phoenix {
    link: AgentLink<Phoenix>,
    phoenix_service: PhoenixService,
    subscribers: HashSet<HandlerId>,
}

impl Agent for Phoenix {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = Socket;

    fn create(link: AgentLink<Self>) -> Self {
        let mut phoenix_service = PhoenixService::new();

        phoenix_service.connect();

        Phoenix {
            link: link,
            phoenix_service: phoenix_service,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::OnError => {
                info!("Socket error");
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, Socket {});
                }
            }
            Msg::OnClose => {
                info!("Socket closed");
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, Socket {});
                }
            }
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            Request::Join(_channel) => {
                ()
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

struct PhoenixService {
    socket: Option<Value>
}

impl PhoenixService {
    pub fn new() -> Self {
        let socket = js! {
            var socket = new Phoenix.Socket("ws://localhost:4000/socket");
            return socket;
        };
        PhoenixService {
            socket: Some(socket)
        }
    }

    pub fn connect(&mut self) {
        let socket = self.socket.as_ref().expect("socket object lost");
        js! {
            var socket = @{socket};
            socket.connect();
        }
    }
}
