use yew::prelude::worker::*;
use yew::services::ConsoleService;
use yew::callback::Callback;

use std::collections::HashSet;
use std::collections::HashMap;

use stdweb::Value;

use serde::Serialize;
use serde::Deserialize;

//
// Phoenix
//

pub enum Msg {
    OnSocketError,
    OnSocketClose,
    // OnChannelError(Channel),
    // OnChannelClose(Channel),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Join(String),
}

impl Transferable for Request {}

#[derive(Serialize, Deserialize)]
pub struct SocketMessage {}

impl Transferable for SocketMessage {}

pub struct Phoenix {
    link: AgentLink<Phoenix>,
    phoenix_service: PhoenixService,
    subscribers: HashSet<HandlerId>,
}

impl Agent for Phoenix {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = SocketMessage;

    fn create(link: AgentLink<Self>) -> Self {
        let mut phoenix_service = PhoenixService::new();

        phoenix_service.connect();

        let callback = link.send_back(|event: SocketEvent| {
            match event {
                SocketEvent::OnError => { Msg::OnSocketError }
                SocketEvent::OnClose => { Msg::OnSocketClose }
            }
        });
        phoenix_service.register_socket_callback(callback);

        Phoenix {
            link: link,
            phoenix_service: phoenix_service,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::OnSocketError => {
                info!("Socket error");
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, SocketMessage {});
                }
            }
            Msg::OnSocketClose => {
                info!("Socket closed");
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, SocketMessage {});
                }
            }
            // Msg::OnChannelError(channel) => {
            //     info!("Channel error");
            //     for sub in self.subscribers.iter() {
            //         self.link.response(*sub, SocketMessage {});
            //     }
            // }
            // Msg::OnChannelClose(channel) => {
            //     info!("Channel closed");
            //     for sub in self.subscribers.iter() {
            //         self.link.response(*sub, SocketMessage {});
            //     }
            // }
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            Request::Join(channel) => {
                self.phoenix_service.join(channel);
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


//
// Service
//

struct Channel {}

enum SocketEvent {
    OnError,
    OnClose,
}

struct Socket {
    js_socket: Option<Value>,
}

struct PhoenixService {
    socket: Socket,
    channels: HashMap<String, Channel>,
}

impl PhoenixService {
    pub fn new() -> Self {
        let socket = js! {
            var socket = new Phoenix.Socket("ws://localhost:4000/socket");
            return socket;
        };

        let mut channels = HashMap::new();

        PhoenixService {
            socket: Socket { js_socket: Some(socket) },
            channels: channels,
        }
    }

    pub fn connect(&mut self) {
        let socket = self.socket.js_socket.as_ref().expect("socket object lost");
        js! {
            var socket = @{socket};
            socket.connect();
        }
    }

    pub fn join(&mut self, channel_name: String) {
        if !self.channels.contains_key(&channel_name) {
            let socket = self.socket.js_socket.as_ref().expect("socket object lost");
            let new_channel_name = channel_name.clone();
            let new_channel = js! {
                var socket = @{socket};
                var channel = socket.channel(@{new_channel_name});
                return channel;
            };
            self.channels.insert(channel_name.to_string(), Channel {});
        }
    }

    pub fn register_socket_callback(&mut self, registering_callback: Callback<SocketEvent>) {
        let error = "error".to_owned();
        let close = "close".to_owned();

        let callback = move |event: String| {
            match event {
                error => { registering_callback.emit(SocketEvent::OnError); }
                close => { registering_callback.emit(SocketEvent::OnClose); }
            }
        };

        let socket = self.socket.js_socket.as_ref().expect("socket object lost");

        js! {
            var socket = @{socket};
            var callback = @{callback};
            var errorString = @{error};
            var closeString = @{close};

            socket.onError(function() {
                callback(errorString);
            });
            socket.onClose(function() {
                callback(closeString);
            });
        }
    }
}
